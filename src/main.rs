use std::net::SocketAddr;

use std::sync::Arc;
use anyhow::{Result};
use sqlx::postgres::PgPoolOptions;
use teloxide::{utils::command::BotCommands, dispatching::update_listeners::webhooks, prelude::*};
use crate::models::test::Test;
use crate::services::TestService;
use crate::sqlx_clients::SqlxClient;
use crate::models::enums::Command;

mod models;
mod api;
mod sqlx_clients;
mod services;

#[tokio::main]
async fn main() -> Result<()> {
    println!("->> It is alive!");

    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    //Database
    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable connect to Postgres");

    sqlx::migrate!().run(&pool).await?;

    let sqlx_client = SqlxClient::new(pool);

    let test_service = Arc::new(TestService::new(
        sqlx_client.clone(),
    ));

    //Telegram bot
    let bot = Bot::from_env();

    let ngrok_host = ([127, 0, 0, 1], 8000).into();//Move port to config
    let url = "https://c8ed-178-253-198-174.ngrok-free.app".parse().unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(ngrok_host, url))
        .await
        .expect("Couldn't setup webhook");
    println!("->> LISTENING Telegram bot requests on {ngrok_host}\n");
    Command::repl_with_listener(bot, handler, listener).await;
    // Command::repl(bot, handler).await;


    //Router
    let router = api::routers::router(test_service);

    //Server
    let server_address = SocketAddr::from(([127, 0, 0, 1], 8080));//Move port to config

    axum::Server::bind(&server_address)
        .serve(router.into_make_service())
        .await
        .unwrap();
    println!("->> LISTENING HTTP requests on {server_address}\n");
    Ok(())
}

async fn handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    let user_name = &(msg.from().unwrap()).first_name;

    match cmd {
        Command::Start => bot.send_message(chat_id, format!("Hi {user_name}")).await?,//Add more text
        Command::Usd(text) => {
            bot.send_message(chat_id, format!("USD is: {text}")).await?
        },
        Command::Eur(text) => {
            bot.send_message(chat_id, format!("EUR is: {text}")).await?
        },
        Command::Feedback(text) => {
            bot.send_message(chat_id, format!("Feedback is: {text}")).await?
        },
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
    };

    Ok(())
}