mod routing;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::convert::Infallible;
use std::net::SocketAddr;

pub struct KeeperConfig {
    server_port: u16,
}

impl KeeperConfig {
    pub fn new(server_port: u16) -> KeeperConfig {
        KeeperConfig { server_port }
    }
}

pub async fn run(config: &KeeperConfig) {
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));

    let make_service =
        make_service_fn(|_| async { Ok::<_, Infallible>(service_fn(routing::routes)) });

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
