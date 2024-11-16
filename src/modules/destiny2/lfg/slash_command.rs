use async_trait::async_trait;
use serenity::all::{CommandInteraction, Context, CreateCommand, Ready};
use sqlx::Postgres;
use zayden_core::{Autocomplete, SlashCommand};

use crate::{sqlx_lib::PostgresPool, Error, Result};

use super::{LfgPostTable, UsersTable};

pub struct LfgCommand;

#[async_trait]
impl SlashCommand<Error> for LfgCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        interaction.defer_ephemeral(ctx).await?;

        let pool = PostgresPool::get(ctx).await;

        lfg::LfgCommand::run::<Postgres, LfgPostTable, UsersTable>(ctx, interaction, &pool).await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(lfg::LfgCommand::register())
    }
}

#[async_trait]
impl Autocomplete<Error> for LfgCommand {
    async fn autocomplete(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        lfg::LfgCommand::autocomplete(ctx, interaction).await?;

        Ok(())
    }
}
