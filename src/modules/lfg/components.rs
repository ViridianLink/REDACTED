use serenity::all::{ComponentInteraction, Context};
use sqlx::Postgres;

use crate::sqlx_lib::PostgresPool;
use crate::Result;

use super::LfgPostTable;

pub struct LfgComponents;

impl LfgComponents {
    pub async fn join(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        lfg::PostComponents::join::<Postgres, LfgPostTable>(ctx, interaction, &pool).await?;

        Ok(())
    }

    pub async fn leave(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        lfg::PostComponents::leave::<Postgres, LfgPostTable>(ctx, interaction, &pool).await?;

        Ok(())
    }
}
