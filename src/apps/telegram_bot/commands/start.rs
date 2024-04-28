use teloxide::prelude::*;

use crate::constants::GREETINGS_TEXT;

pub async fn handle_start(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, GREETINGS_TEXT).await?;
    Ok(())
}
