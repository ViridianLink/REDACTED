// mod chatgpt_lib;
// pub mod components;
// pub mod cron;
mod error;
// mod global_commands;
// mod guild_commands;
// pub mod guilds;
mod handler;
// mod image_cache;
// pub mod modals;
// mod models;
pub mod modules;
mod sqlx_lib;
// mod sqlx_lib;
// pub mod state;
// mod utils;

// use guild_commands::college_kings::{
//     goodmorning::GoodMorningLockedUsers, goodnight::GoodNightLockedUsers,
// };
use serenity::all::{GatewayIntents, UserId};
use serenity::Client;
use sqlx_lib::PostgresPool;
use std::collections::HashMap;
use std::env;
use temp_voice::VoiceStateCache;

// use crate::image_cache::ImageCache;
pub use error::{Error, Result};

// pub const SERVER_URL: &str = "http://145.40.184.89:8080";
pub const OSCAR_SIX_ID: UserId = UserId::new(211486447369322506);

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let token = &env::var("DISCORD_TOKEN")?;

    let mut client = Client::builder(token, GatewayIntents::all())
        .raw_event_handler(handler::Handler)
        .await?;

    let mut data = client.data.write().await;
    data.insert::<PostgresPool>(PostgresPool::init().await?);
    data.insert::<VoiceStateCache>(HashMap::new());
    drop(data);

    client.start().await?;

    Ok(())
}
