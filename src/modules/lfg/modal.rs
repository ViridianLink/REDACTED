use serenity::all::{Context, CreateInteractionResponse, ModalInteraction};
use sqlx::Postgres;

use crate::sqlx_lib::PostgresPool;
use crate::Result;

use super::LfgPostTable;

pub struct LfgCreateModal;

impl LfgCreateModal {
    pub async fn run(ctx: &Context, interaction: &ModalInteraction) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        lfg::LfgCreateModal::run::<Postgres, LfgPostTable>(ctx, interaction, &pool).await?;

        interaction
            .create_response(ctx, CreateInteractionResponse::Acknowledge)
            .await?;

        Ok(())
    }
}

pub struct LfgEditModal;

impl LfgEditModal {
    pub async fn run(ctx: &Context, interaction: &ModalInteraction) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        lfg::LfgEditModal::run::<Postgres, LfgPostTable>(ctx, interaction, &pool).await?;

        interaction
            .create_response(ctx, CreateInteractionResponse::Acknowledge)
            .await?;

        Ok(())
    }
}
