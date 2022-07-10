use disruption::Client;
use log::trace;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let mut client = Client::new(env::var("BOT_TOKEN")?.to_owned()).await?;

    if let Err(e) = client.start().await {
        trace!("{}", e);
    }
    Ok(())
}
