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
        // We get a fresh database connexion
        let database_conn = self.database.get_conn()?;
        // We persist the fresh nodes as not "visited"
        let new_nodes: Vec<models::NodeForm> = crawling_results
            .urls
            .iter()
            .map(|url| models::NodeForm {
                node: String::from(url),
                visited: false,
            })
            .collect();
        self.database
            .insert_nodes(&database_conn, new_nodes)
            .await?;
        // We set the parent to "visited"
        let visited_node = models::NodeForm {
            node: crawling_results.parent.clone(),
            visited: true,
        };
        self.database
            .update_node(&database_conn, visited_node)
            .await?;
        // We add the fresh nodes parent relations
        let new_parents: Vec<models::ParentForm> = crawling_results
            .urls
            .iter()
            .map(|url| models::ParentForm {
                parent: crawling_results.parent.clone(),
                node: String::from(url),
            })
            .collect();
        self.database
            .insert_parents(&database_conn, new_parents)
            .await?;
        Ok(())
    }
}

#[derive(Deserialize)]
struct CrawlingResults {
    parent: String,
    urls: Vec<String>,
}
