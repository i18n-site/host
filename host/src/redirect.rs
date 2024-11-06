use async_trait::async_trait;
use http::{header, Response, StatusCode};
use pingora::{
  apps::http_app::ServeHttp, protocols::http::ServerSession, services::listening::Service,
};
use tracing::info;
pub struct Redirect;

#[async_trait]
impl ServeHttp for Redirect {
  async fn response(&self, stream: &mut ServerSession) -> Response<Vec<u8>> {
    let header = stream.req_header();

    let builder = Response::builder()
      .status(StatusCode::MOVED_PERMANENTLY)
      .header(header::CONTENT_LENGTH, 0);

    if let Some(host) = header.headers.get(header::HOST)
      && let Ok(host) = host.to_str()
    {
      let uri = &header.uri;
      let url = format!("https://{}{}", host, uri);
      info!("{}", url);
      builder.header(header::LOCATION, url)
    } else {
      builder.header(header::LOCATION, "https://i18n.site")
    }
    .body(vec![])
    .unwrap_or_default()
  }
}

pub fn service() -> Service<Redirect> {
  let mut service = Service::new("https redirect".into(), Redirect {});
  service.add_tcp(":::80");
  service.add_tcp("0.0.0.0:80");
  service
}
