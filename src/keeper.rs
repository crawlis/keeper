use crate::nats::NatsSubscriber;
use crate::persistence::database;
use crate::persistence::models;
use serde::Deserialize;
use std::error::Error;
use std::time;

pub struct KeeperConfig {
    nats_subscriber_uri: String,
    nats_subscriber_subject: String,
    database_uri: String,
}

impl KeeperConfig {
    pub fn new(
        nats_subscriber_uri: String,
        nats_subscriber_subject: String,
        database_uri: String,
    ) -> KeeperConfig {
        KeeperConfig {
            nats_subscriber_uri,
            nats_subscriber_subject,
            database_uri,
        }
    }
}

pub struct Keeper {
    config: KeeperConfig,
    nats_subscriber: NatsSubscriber,
    database: database::Database,
}

impl Keeper {
    pub fn new(config: KeeperConfig) -> Result<Keeper, std::io::Error> {
        let nats_subscriber =
            NatsSubscriber::new(&config.nats_subscriber_uri, &config.nats_subscriber_subject)?;
        let database = database::Database::new(&config.database_uri);
        Ok(Keeper {
            config,
            nats_subscriber,
            database,
        })
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        self.database
            .wait_for_conn(time::Duration::from_secs(2), 10)?;
        self.database.run_migrations()?;

        loop {
            if let Some(message) = self.nats_subscriber.get_next_message() {
                match serde_json::from_slice::<CrawlingResults>(&message.data) {
                    Ok(crawling_results) => {
                        if let Err(err) = self.persist_crawling_results(crawling_results).await {
                            eprintln!("Could not persist nodes: {}", err);
                        };
                    }
                    Err(err) => eprintln!("Could not deserialize message: {}", err),
                }
            }
        }

        Ok(())
    }

    async fn persist_crawling_results(
        &self,
        crawling_results: CrawlingResults,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let new_nodes: Vec<models::NewNode> = crawling_results
            .urls
            .iter()
            .map(|url| models::NewNode {
                parent: crawling_results.parent.clone(),
                value: String::from(url),
            })
            .collect();
        self.database.insert_nodes(new_nodes).await
    }
}

#[derive(Deserialize)]
struct CrawlingResults {
    parent: String,
    urls: Vec<String>,
}
