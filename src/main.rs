mod crack;
mod telegram;
mod discord;
use crack::*;

#[macro_use] extern crate log;
extern crate pretty_env_logger;


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv::dotenv().ok();
    info!("Starting Hash Cracker Bot...");
    
    tokio::spawn(async move {
        info!("Starting telegram bot...");
        telegram::start().await;
    });
    tokio::spawn(async move {
        info!("Starting discord bot...");
        discord::start().await;
    });

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

