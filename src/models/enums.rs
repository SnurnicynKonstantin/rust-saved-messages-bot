use serde::Serialize;
use teloxide::{utils::command::BotCommands};

#[derive(Serialize)]
pub enum ResponseStatus {
    Ok,
    Error,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These bot supported next commands:")]
pub enum Command {
    #[command(description = "Command for print list of all commands.")]
    Help,
    #[command(description = "Register user in system.")]
    Start,
    #[command(description = "Get USD course for target RUB summ. Example of command \"/usd 123\"")]
    Usd(String),
    #[command(description = "Get EUR course for target RUB summ. Example of command \"/eur 123\"")]
    Eur(String),
    #[command(description = "Send feedback for owner of this bot. Example of command \"/feedback It is very useful boot!\"")]
    Feedback(String),
}