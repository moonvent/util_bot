use teloxide::{prelude::*, utils::command::BotCommands};

use super::commands::all_commands::Command;
use super::commands::start::handle_start;

use dotenv::dotenv;

pub async fn init_bot() {
    dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    // teloxide::repl(bot, |bot: Bot, msg: Message| async move {
    //     bot.send_dice(msg.chat.id).await?;
    //     Ok(())
    // })
    // .await;

    teloxide::commands_repl(bot, handle_command, Command::ty()).await;
}

async fn handle_command(bot: Bot, msg: Message, command: Command) -> ResponseResult<()> {
    match command {
        Command::Start => handle_start(&bot, &msg).await,
        Command::Help => Ok(()),
    }
}
