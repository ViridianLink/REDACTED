mod error;

mod handler;

pub mod modules;
mod sqlx_lib;

use modules::destiny2::dimwishlist::bungie_api::BungieApi;
use serenity::all::{GatewayIntents, UserId};
use serenity::prelude::TypeMap;
use serenity::Client;
use sqlx_lib::PostgresPool;
use std::collections::HashMap;
use std::env;
use temp_voice::VoiceStateCache;

pub use error::{Error, Result};

pub const OSCAR_SIX_ID: UserId = UserId::new(211486447369322506);

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().unwrap();

    let pool = PostgresPool::init().await?;

    BungieApi::update_dbs(&pool).await.unwrap();

    let mut type_map = TypeMap::new();
    type_map.insert::<PostgresPool>(pool);
    type_map.insert::<VoiceStateCache>(HashMap::new());

    let token = &env::var("DISCORD_TOKEN").unwrap();

    let mut client = Client::builder(token, GatewayIntents::all())
        .type_map(type_map)
        .raw_event_handler(handler::Handler)
        .await
        .unwrap();

    client.start().await.unwrap();

    Ok(())
}
