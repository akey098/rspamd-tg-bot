use std::error::Error;
use teloxide::prelude::*;
use rspamd_client::{config::Config};
use std::sync::Arc;

use crate::handlers::scan_msg;

pub async fn handle_message(
    bot: Bot,
    message: Message,
    options: Arc<Config>,
) -> Result<(), Box<dyn Error + Send + Sync>> {

    let text = if let Some(text) = message.text() {
        text.to_string()
    } else {
        return Ok(());
    };

    match scan_msg::scan_msg(&options, message.clone(), text.clone()).await {
        Ok(reply) => {
            let spam_threshold = 0.0;
            if reply.score >= spam_threshold {
                bot.send_message(message.chat.id, format!(
                    "Warning @{}: your message was detected as {} (score: {}).",
                    message.from.unwrap().username.unwrap().to_string(), reply.action, reply.score
                ))
                    .await?;
            }
        }
        Err(err) => {
            eprintln!("Error scanning message: {:?}", err);
        }
    }

    Ok(())
}
