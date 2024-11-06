use async_trait::async_trait;
use http::{header, Response, StatusCode};
use pingora::{
  apps::http_app::ServeHttp, protocols::http::ServerSession, services::listening::Service,
};
use tracing::info;
pub struct Redirect;

pub const ACME_CHALLENGE: &str = ".well-known/acme-challenge/";
pub const ACME_CHALLENGE_LEN: usize = ACME_CHALLENGE.len() + 1;

#[async_trait]
impl ServeHttp for Redirect {
  async fn response(&self, stream: &mut ServerSession) -> Response<Vec<u8>> {
    let header = stream.req_header();

    let mut builder = Response::builder();

    let body;
    let status;

    #[allow(clippy::never_loop)]
    'out: loop {
      builder = if let Some(host) = header.headers.get(header::HOST)
        && let Ok(host) = host.to_str()
      {
        let uri = header.uri.to_string();
        if uri[1..].starts_with(ACME_CHALLENGE) && uri.len() > ACME_CHALLENGE_LEN {
          let uri = &uri[ACME_CHALLENGE_LEN..];
          body = uri.as_bytes().to_vec();
          status = StatusCode::OK;
          break 'out;
        }
        let url = format!("https://{}{}", host, uri);
        info!("{}", url);
        builder.header(header::LOCATION, url)
      } else {
        builder.header(header::LOCATION, "https://i18n.site")
      };
      body = vec![];
      status = StatusCode::MOVED_PERMANENTLY;
      break;
    }

    builder
      .status(status)
      .header(header::CONTENT_LENGTH, body.len())
      .body(body)
      .unwrap_or_default()
  }
}

pub fn service() -> Service<Redirect> {
  let mut service = Service::new("https redirect".into(), Redirect {});
  for ip in [":::80", "0.0.0.0:80"] {
    info!("listen {}", ip);
    service.add_tcp(ip);
  }
  service
}
