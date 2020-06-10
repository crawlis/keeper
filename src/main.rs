use dotenv::dotenv;
use keeper::keeper::{Keeper, KeeperConfig};
use std::env;
use std::error::Error;
use std::process;
use url::Url;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = get_config().unwrap_or_else(|err| {
        eprintln!("Problem initializing keeper config: {}", err);
        process::exit(1);
    });
    let keeper = Keeper::new(config).unwrap_or_else(|err| {
        eprintln!("Problem initializing keeper: {}", err);
        process::exit(1);
    });

    keeper.run().await.unwrap_or_else(|err| {
        eprintln!("Problem running the keeper: {}", err);
        panic!("Terminating");
    });
}

fn get_config() -> Result<KeeperConfig, Box<dyn Error>> {
    let nats_uri = env::var("NATS_URI")?;
    let nats_uri = Url::parse(&nats_uri)?;

    let nats_subject = String::from("node");

    let database_uri = env::var("DATABASE_URI")?;
    let database_uri = Url::parse(&database_uri)?;

    let config = KeeperConfig::new(
        nats_uri.into_string(),
        nats_subject,
        database_uri.into_string(),
    );
    Ok(config)
}
