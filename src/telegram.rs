use std::sync::Arc;
use anyhow::{anyhow, Result};
use teloxide::{utils::command::BotCommands, prelude::*};
use teloxide::types::Me;
use crate::{Account, HttpClient};
use crate::models::enums::Command;
use crate::services::AccountService;

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
    account_service: Arc<AccountService>,
    http_client: Arc<HttpClient>
) -> Result<()> {
    match BotCommands::parse(msg.text().unwrap(), me.username()) {
        Ok(command) => {
            process(command, msg, bot, account_service, http_client).await?;
        }
        Err(_) => {
            println!("Error happened!");
        }
    }
    Ok(())
}

async fn process(
    command: Command,
    msg: Message,
    bot: Bot,
    account_service: Arc<AccountService>,
    http_client: Arc<HttpClient>
) -> Result<()> {
    let chat_id = msg.chat.id;

    match command {
        Command::Start => bot.send_message(chat_id, create_account(msg, account_service).await).await?,
        Command::Usd(text) => {
            match parse_and_validate_value(text) {
                Ok(parsed_value) => {
                    let course = get_usd_course(parsed_value, http_client).await;
                    bot.send_message(chat_id, format!("According to current course {parsed_value} RUB equals {course} USD")).await?
                },
                Err(err) => bot.send_message(chat_id, err).await?,
            }
        },
        Command::Eur(text) => {
            match parse_and_validate_value(text) {
                Ok(parsed_value) => {
                    let course = get_eur_course(parsed_value, http_client).await;
                    bot.send_message(chat_id, format!("According to current course {parsed_value} RUB equals {course} EUR")).await?
                },
                Err(err) => bot.send_message(chat_id, err).await?,
            }
        },
        Command::Feedback(text) => {
            bot.send_message(chat_id, format!("Feedback is: {text}")).await?
        },
        Command::Help => bot.send_message(chat_id, Command::descriptions().to_string()).await?,
    };
    Ok(())
}

pub async fn get_usd_course(amount: f32, http_client: Arc<HttpClient>) -> f32 {
    http_client.get_course("RUB".to_string(), "USD".to_string(), amount).await.unwrap()
}

pub async fn get_eur_course(amount: f32, http_client: Arc<HttpClient>) -> f32 {
    http_client.get_course("RUB".to_string(), "EUR".to_string(), amount).await.unwrap()
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

fn parse_and_validate_value(value: String) -> Result<f32, &'static str>{
    match value.replace(",", ".").parse() {
        Ok(parsed_value) => {
            if parsed_value > 0.0 {
                Ok(parsed_value)
            } else {
                Err("Value must be more than zero.")
            }
        },
        Err(_) => Err("Please enter digit.")
    }
}