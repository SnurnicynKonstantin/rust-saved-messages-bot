use std::sync::Arc;
use anyhow::{Result};
use teloxide::{utils::command::BotCommands, prelude::*};
use teloxide::types::Me;
use crate::models::enums::Command;
use crate::services::AccountService;

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
    account_service: Arc<AccountService>
) -> Result<()> {
    // let f: String = msg.from().unwrap().first_name.to_string();
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(command) => {
                process(command, msg, bot, account_service).await?;
            }
            Err(_) => {
                // send_error_message("command_not_found", msg.chat, bot, &pool, &translation).await?;
                println!("Error!");
            }
        }
    } else {
        // send_error_message("text_expected", msg.chat, bot, &pool, &translation).await?;
        println!("Error!");
    }
    Ok(())
}

async fn process(
    command: Command,
    msg: Message,
    bot: Bot,
    account_service: Arc<AccountService>
) -> Result<()> {
    let chat_id = msg.chat.id;
    let account_name = "Hi hi";//&(msg.from().unwrap()).first_name;

    match command {
        Command::Start => bot.send_message(chat_id, format!("Hi {account_name}")).await?,//Add more text
        Command::Usd(text) => {
            let course = get_usd_course(text, account_service).await;
            bot.send_message(chat_id, format!("USD is: {course}")).await?
        },
        Command::Eur(text) => {
            bot.send_message(chat_id, format!("EUR is: {text}")).await?
        },
        Command::Feedback(text) => {
            bot.send_message(chat_id, format!("Feedback is: {text}")).await?
        },
        Command::Help => bot.send_message(chat_id, Command::descriptions().to_string()).await?,
    };
    Ok(())
}

pub async fn get_usd_course(text: String, account_service: Arc<AccountService>) -> String {
    // let users = test_service.get_tests().await;

    let accounts = account_service
        .get_accounts()
        .await;

    println!("{:?}", accounts);
    "123".to_owned()
}

pub async fn save_account_data(text: String, account_service: Arc<AccountService>) -> String {
    // let users = test_service.get_tests().await;

    let accounts = account_service
        .get_accounts()
        .await;

    println!("{:?}", accounts);
    "123".to_owned()
}