#![feature(let_chains)]

mod redirect;

use aok::Null;
use pingora::prelude::Server;

fn main() -> Null {
  let mut srv = Server::new(None)?;
  srv.bootstrap();
  srv.add_services(vec![Box::new(redirect::service())]);
  srv.run_forever();
}
