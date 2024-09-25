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
mod client;
pub mod modules;
mod sqlx_lib;
// mod sqlx_lib;
// pub mod state;
// mod utils;

pub use client::Client;
use handler::handle_event;
// use guild_commands::college_kings::{
//     goodmorning::GoodMorningLockedUsers, goodnight::GoodNightLockedUsers,
// };
use serenity::all::UserId;
// use sqlx::postgres::PgPoolOptions;
// use sqlx_lib::PostgresPool;
// use state::State;
use std::env;

// use crate::image_cache::ImageCache;
pub use error::{Error, Result};

// pub const SERVER_URL: &str = "http://145.40.184.89:8080";
pub const OSCAR_SIX_ID: UserId = UserId::new(211486447369322506);

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;

    let client = Client::new(token);

    start_loop(client).await;

    Ok(())
}

async fn start_loop(client: Client) {
    while let item = client.shard.next_event().await {
        let Ok(event) = item else {
            tracing::warn!(source = ?item.unwrap_err(), "error receiving event");

            continue;
        };

        client.cache.update(&event);

        tokio::spawn(handle_event(client, event));
    }
}
