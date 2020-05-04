use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::Filter;

pub struct KeeperConfig {
    server_port: u16,
}

impl KeeperConfig {
    pub fn new(server_port: u16) -> KeeperConfig {
        KeeperConfig { server_port }
    }
}

pub struct Keeper {
    server_port: u16,
}

impl Keeper {
    pub fn new(config: KeeperConfig) -> Keeper {
        Keeper {
            server_port: config.server_port,
        }
    }

    pub async fn run(&self) {
        let routes = warp::post()
            .and(warp::any())
            .and(warp::body::json())
            .and_then(Keeper::persist_message);

        println!("Server listening on 127.0.0.1:{}", self.server_port);
        warp::serve(routes)
            .run(([0, 0, 0, 0], self.server_port))
            .await;
    }

    async fn persist_message(
        message: CrawlerResultsMessage,
    ) -> Result<impl warp::Reply, Infallible> {
        println!("Received results from {}", message.parent);
        Ok(warp::reply())
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct CrawlerResultsMessage {
    parent: String,
    children: Vec<String>,
}
