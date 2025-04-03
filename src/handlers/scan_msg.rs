use teloxide::prelude::*;
use rspamd_client::{config::Config, scan_async, error::RspamdError, protocol::RspamdScanReply};

pub async fn scan_msg(options: &Config, msg: Message, text: String) -> Result<RspamdScanReply, RspamdError> {
    let user = msg.from.unwrap();
    let username = user.username.unwrap();
    let user_id = user.id.to_string();
    let date = msg.date.to_string();
    let chat_id = msg.chat.id.to_string();
    let message_id = msg.id.to_string();
    let text = text;
    let email = format!("From: {username}\nSent: {date}\nTo: {chat_id}\n\
    Subject: {message_id}\nUser: {user_id}\n\n{text}");
    scan_async(options, email).await
}
