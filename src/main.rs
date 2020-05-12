extern crate openssl;

#[macro_use]
extern crate diesel;

mod persistence;
mod routing;

use dotenv::dotenv;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use persistence::database::Database;
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use std::process;

struct KeeperConfig {
    server_port: u16,
    db_url: String,
}

impl KeeperConfig {
    pub fn new(server_port: u16, db_url: String) -> KeeperConfig {
        KeeperConfig {
            server_port,
            db_url,
        }
    }
}

async fn run(config: &KeeperConfig) {
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    let database = Database::new(&config.db_url);
    let make_service = make_service_fn(move |_| {
        let database = database.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let database = database.clone();
                async move { routing::routes(req, &database).await }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

fn get_config() -> Result<KeeperConfig, Box<dyn Error>> {
    let server_port = env::var("SERVER_PORT")?;
    let server_port = server_port.parse::<u16>()?;
    let db_url = env::var("DATABASE_URL")?;

    let config = KeeperConfig::new(server_port, db_url);
    Ok(config)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = get_config().unwrap_or_else(|err| {
        eprintln!("Problem initializing keeper config: {}", err);
        process::exit(1);
    });

    run(&config).await;
}
