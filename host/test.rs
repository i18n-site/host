// use std::fs;
// use std::path::Path;
// use std::time::SystemTime;
//
// use pingora::Pingora;
// use pingora::listener::HttpsListener;
// use pingora::middleware::{Middleware, Next};
// use async_trait::async_trait;
// use rustls::{Certificate, PrivateKey};
// use rustls_pemfile::{certs, rsa_private_keys};
// use acme_client::Client;
// use acme_client::error::Error as AcmeError;
//
// struct CertificateMiddleware {
//     client: Client,
// }
//
// #[async_trait]
// impl Middleware for CertificateMiddleware {
//     async fn handle(&self, ctx: pingora::Context, next: Next) -> pingora::Response {
//         let domain = ctx.req().headers().get("host").unwrap().to_str().unwrap();
//         let cert_path = format!("/mnt/ssl/{}.crt", domain);
//         let key_path = format!("/mnt/ssl/{}.key", domain);
//
//         if Path::new(&cert_path).exists() && Path::new(&key_path).exists() {
//             let cert = fs::read(cert_path).unwrap();
//             let key = fs::read(key_path).unwrap();
//             let cert = certs(&mut cert.as_slice()).unwrap();
//             let key = rsa_private_keys(&mut key.as_slice()).unwrap();
//             let cert = Certificate(cert[0].clone());
//             let key = PrivateKey(key[0].clone());
//
//             // 使用现有的证书响应请求
//             let https_listener = HttpsListener::new(cert, key);
//             let res = next.run(ctx, https_listener).await;
//             return res;
//         } else {
//             // 申请新证书
//             let dir_url = "https://acme-staging-v02.api.letsencrypt.org/directory";
//             let client = self.client.clone();
//             let account = client.new_account(None).await.unwrap();
//             let order = client.new_order(&account, &[domain]).await.unwrap();
//             let auth = order.authorizations().await.unwrap();
//             let challenge = auth[0].http_challenge();
//             let token = challenge.http_token();
//             let validation = challenge.http_validation();
//
//             // 验证域名所有权
//             let validation_path = format!("/.well-known/acme-challenge/{}", token);
//             let validation_content = validation.to_string();
//             // 写入验证文件
//             fs::write(validation_path, validation_content).unwrap();
//
//             // 请求验证
//             challenge.validate().await.unwrap();
//
//             // 获取证书
//             let cert = order.certificate().await.unwrap();
//             let cert = cert.certificate().to_pem().unwrap();
//             let key = cert.private_key().to_pem().unwrap();
//
//             // 保存证书到本地
//             fs::write(cert_path, cert).unwrap();
//             fs::write(key_path, key).unwrap();
//
//             // 打印证书过期日期
//             let cert = certs(&mut cert.as_slice()).unwrap();
//             let cert = Certificate(cert[0].clone());
//             let expires_at = cert.expires_at();
//             println!("证书过期日期：{:?}", expires_at);
//
//             // 使用新证书响应请求
//             let https_listener = HttpsListener::new(cert, key);
//             let res = next.run(ctx, https_listener).await;
//             return res;
//         }
//     }
// }
//
// #[tokio::main]
// async fn main() {
//     let client = Client::new();
//     let middleware = CertificateMiddleware { client };
//     let mut pingora = Pingora::new();
//     pingora.use(middleware);
//     pingora.listen(443).await.unwrap();
// }
