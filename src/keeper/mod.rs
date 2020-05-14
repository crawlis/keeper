mod persistence;
mod routing;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use persistence::database::Database;
use std::error::Error;
use std::net::SocketAddr;

pub struct KeeperConfig {
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

pub struct Keeper {
    config: KeeperConfig,
}

impl Keeper {
    pub fn new(config: KeeperConfig) -> Keeper {
        Keeper { config }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let addr = SocketAddr::from(([0, 0, 0, 0], self.config.server_port));
        let database = Database::new(&self.config.db_url);
        database.run_migrations()?;
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

        let shutdown = server.with_graceful_shutdown(shutdown_signal());

        if let Err(e) = shutdown.await {
            eprintln!("server error: {}", e);
        }

        Ok(())
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}
