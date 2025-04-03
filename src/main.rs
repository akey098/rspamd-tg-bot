use teloxide::prelude::*;
use rspamd_client::{config::Config};
use std::sync::Arc;
mod handlers;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting the spam detection bot...");

    let bot = Bot::from_env();

    let options = Arc::new(Config::builder()
        .base_url("http://localhost:11333".to_string())
        .build());

    teloxide::repl(bot, move |bot: Bot, message: Message| {
        let options = Arc::clone(&options);
        async move {
            if let Err(e) = handlers::handle_message(bot.clone(), message, options).await {
                eprintln!("Error handling message: {:?}", e);
            }
            respond(())
        }
    })
        .await;
}
