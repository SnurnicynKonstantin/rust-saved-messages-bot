use std::sync::Arc;
use anyhow::{Result};
use teloxide::{utils::command::BotCommands, prelude::*};
use teloxide::types::Me;
use crate::Account;
use crate::models::enums::Command;
use crate::services::AccountService;

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
    account_service: Arc<AccountService>
) -> Result<()> {
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

    match command {
        Command::Start => bot.send_message(chat_id, create_account(msg, account_service).await).await?,
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

pub async fn create_account(msg: Message, account_service: Arc<AccountService>) -> String {
    let user_data = msg.from().unwrap();
    //TODO Replace clone() on work with links
    let account = Account {id: 0, name: user_data.clone().first_name, surname: user_data.clone().last_name.unwrap(), user_id: user_data.clone().id.0 as i64};


    let account = account_service
        .save_account(account)
        .await;

    format!("Hi {}! \nI'm a bot for exchange rate RUB -> USD or RUB -> EUR! \nYou can use /help command for get mode info. \nHave nice day)", account.unwrap().name)
}