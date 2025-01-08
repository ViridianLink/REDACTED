use async_trait::async_trait;
use serenity::all::{CommandInteraction, Context, CreateCommand, Ready, ResolvedOption};
use sqlx::Postgres;
use zayden_core::SlashCommand;

use crate::sqlx_lib::{GuildTable, PostgresPool};
use crate::{Error, Result};

use super::VoiceChannelTable;

pub struct VoiceCommand;

#[async_trait]
impl SlashCommand<Error> for VoiceCommand {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
    ) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        interaction.defer_ephemeral(ctx).await.unwrap();

        temp_voice::VoiceCommand::run::<Postgres, GuildTable, VoiceChannelTable>(
            ctx,
            interaction,
            &pool,
        )
        .await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(temp_voice::VoiceCommand::register())
    }
}
