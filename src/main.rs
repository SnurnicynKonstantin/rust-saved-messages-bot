extern crate core;

use std::net::SocketAddr;

use std::env;
use std::sync::Arc;
use anyhow::{Result};
use teloxide::{prelude::*};
use crate::http_clients::{HttpClient, HttpClientConfig};
use crate::models::account::Account;
use crate::services::{AccountService};
use crate::sqlx_clients::SqlxClient;
use crate::settings::{Config, pool};

mod models;
mod api;
mod sqlx_clients;
mod services;
mod telegram;
mod http_clients;
mod settings;

const TELEGRAM_MODE: &str = "telegram_mode";

#[tokio::main]
async fn main() -> Result<()> {
    let command_line_args: Vec<String> = env::args().collect();
    let mode = command_line_args.get(1);
    println!("->> It is alive!");
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    //Database
    let db_url = env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
    let pool = pool(db_url).await?;
    sqlx::migrate!().run(&pool).await?;
    let sqlx_client = SqlxClient::new(pool);

    //Services
    let config_file_name = env::var("CONFIG_FILE").expect("Unable to read CONFIG_FILE env var");
    let config = Config::new(config_file_name);
    let http_client = Arc::new(HttpClient::new(config.http_client.clone()));
    let account_service = Arc::new(AccountService::new(sqlx_client.clone()));

    if mode.is_some() && TELEGRAM_MODE.eq(mode.unwrap()) {
        //Telegram bot ----------------------
        let bot = Bot::from_env();

        println!("->> LISTENING Telegram bot requests started");

        let handler = dptree::entry()
            .branch(Update::filter_message()
                .endpoint(telegram::message_handler));

        Dispatcher::builder(bot, handler)
            .dependencies(dptree::deps![account_service.clone()])
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    } else {
        //REST API --------------------------
        let router = api::routers::router(account_service);

        //Server
        let server_address = SocketAddr::from(([127, 0, 0, 1], 8080));//Move port to config

        println!("->> LISTENING HTTP requests on {server_address}");

        axum::Server::bind(&server_address)
            .serve(router.into_make_service())
            .await
            .unwrap();
    }

    Ok(())
}