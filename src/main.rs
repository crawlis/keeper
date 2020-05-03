mod keeper;

use dotenv::dotenv;
use keeper::{Keeper, KeeperConfig};
use std::env;
use std::error::Error;
use std::process;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = get_config().unwrap_or_else(|err| {
        eprintln!("Problem initializing crawler config: {}", err);
        process::exit(1);
    });

    let keeper = Keeper::new(config);
    keeper.run().await;
}

fn get_config() -> Result<KeeperConfig, Box<dyn Error>> {
    let server_port = env::var("SERVER_PORT")?;
    let server_port = server_port.parse::<u16>()?;

    let config = KeeperConfig::new(server_port);
    Ok(config)
}
