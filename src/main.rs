mod error;

mod handler;

pub mod modules;
mod sqlx_lib;

use serenity::all::{GatewayIntents, UserId};
use serenity::Client;
use sqlx_lib::PostgresPool;
use std::collections::HashMap;
use std::env;
use temp_voice::VoiceStateCache;

pub use error::{Error, Result};

pub const OSCAR_SIX_ID: UserId = UserId::new(211486447369322506);

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let pool = PostgresPool::init().await?;

    let token = &env::var("DISCORD_TOKEN")?;

    let mut client = Client::builder(token, GatewayIntents::all())
        .raw_event_handler(handler::Handler)
        .await?;

    let mut data = client.data.write().await;
    data.insert::<PostgresPool>(pool);
    data.insert::<VoiceStateCache>(HashMap::new());
    drop(data);

    client.start().await?;

    Ok(())
}
