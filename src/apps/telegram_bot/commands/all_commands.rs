use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    // #[command(alias = "h")]
    Help,
    // #[command(alias = "s")]
    Start,
}
