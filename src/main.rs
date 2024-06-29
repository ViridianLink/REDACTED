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
// mod sqlx_lib;
// pub mod state;
// mod utils;

// use guild_commands::college_kings::{
//     goodmorning::GoodMorningLockedUsers, goodnight::GoodNightLockedUsers,
// };
use serenity::{
    all::{CommandInteraction, Context, CreateCommand, GatewayIntents, UserId},
    async_trait, Client,
};
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
    dotenvy::dotenv()?;

    let token = &env::var("DISCORD_TOKEN")?;

    let mut client = Client::builder(token, GatewayIntents::all())
        .raw_event_handler(handler::Handler)
        .await?;

    let mut data = client.data.write().await;
    // let pool = PgPoolOptions::new()
    //     .max_connections(10)
    //     .min_connections(3)
    //     .connect(&env::var("DATABASE_URL")?)
    //     .await?;
    // data.insert::<State>(State::new());
    // data.insert::<ImageCache>(ImageCache::new());
    // data.insert::<GoodMorningLockedUsers>(Vec::new());
    // data.insert::<GoodNightLockedUsers>(Vec::new());
    // data.insert::<PostgresPool>(pool);
    drop(data);

    client.start().await?;

    Ok(())
}
