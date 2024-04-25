use teloxide::prelude::*;

use dotenv::dotenv;
use std::env;

pub async fn init_bot() {
    dotenv().ok();

    let TELOXIDE_TOKEN = env::var("TELOXIDE_TOKEN").expect("API_KEY must be set");

    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}
