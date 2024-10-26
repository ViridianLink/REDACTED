use async_trait::async_trait;
use serenity::all::{CommandInteraction, Context, CreateCommand};
use sqlx::Postgres;
use zayden_core::SlashCommand;

use crate::{sqlx_lib::PostgresPool, Error, Result};

use super::UsersTable;

pub struct LfgCommand;

#[async_trait]
impl SlashCommand<Error> for LfgCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        interaction.defer_ephemeral(ctx).await?;

        let pool = PostgresPool::get(ctx).await;

        lfg::LfgCommand::run::<Postgres, UsersTable>(ctx, interaction, &pool).await?;

        Ok(())
    }

    fn register() -> CreateCommand {
        lfg::LfgCommand::register()
    }
}
