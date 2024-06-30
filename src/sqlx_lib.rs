use std::env;

use serenity::{all::Context, prelude::TypeMapKey};
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::Result;

pub struct PostgresPool;

impl PostgresPool {
    pub async fn init() -> Result<PgPool> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .min_connections(3)
            .connect(&env::var("DATABASE_URL")?)
            .await?;

        Ok(pool)
    }

    pub async fn get(ctx: &Context) -> PgPool {
        let data = ctx.data.read().await;
        data.get::<PostgresPool>()
            .expect("PostgresPool should exist in data.")
            .clone()
    }
}

impl TypeMapKey for PostgresPool {
    type Value = PgPool;
}
