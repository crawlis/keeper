use crate::nats::NatsSubscriber;
use crate::persistence::database;
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
}

impl Keeper {
    pub fn new(config: KeeperConfig) -> Result<Keeper, std::io::Error> {
        let nats_subscriber =
            NatsSubscriber::new(&config.nats_subscriber_uri, &config.nats_subscriber_subject)?;
        Ok(Keeper {
            config,
            nats_subscriber,
        })
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let database = database::Database::new(&self.config.database_uri);
        database.wait_for_conn(time::Duration::from_secs(2), 10)?;
        database.run_migrations()?;

        loop {}

        Ok(())
    }
}
