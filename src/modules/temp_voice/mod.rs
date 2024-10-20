pub mod events;

use async_trait::async_trait;
use serenity::all::{ChannelId, CommandInteraction, Context, CreateCommand};
use sqlx::{any::AnyQueryResult, Pool, Postgres};
use temp_voice::{PersistentVoiceChannelManager, TemporaryChannelData, VoiceChannelMap};
use zayden_core::SlashCommand;

use crate::{sqlx_lib::PostgresPool, Error, Result};

pub fn register() -> Vec<CreateCommand> {
    vec![VoiceCommand::register()]
}

pub struct VoiceCommand;

#[async_trait]
impl SlashCommand<Error> for VoiceCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        interaction.defer_ephemeral(ctx).await?;

        temp_voice::VoiceCommand::run::<Postgres, VoiceChannelMap, VoiceChannelTable>(
            ctx,
            interaction,
            &pool,
        )
        .await?;

        Ok(())
    }

    fn register() -> CreateCommand {
        temp_voice::VoiceCommand::register()
    }
}

struct VoiceChannelTable;

#[async_trait]
impl PersistentVoiceChannelManager<Postgres> for VoiceChannelTable {
    async fn persist(
        pool: &Pool<Postgres>,
        channel_data: &TemporaryChannelData,
    ) -> sqlx::Result<AnyQueryResult> {
        let trusted_ids = channel_data
            .trusted
            .iter()
            .map(|id| id.get() as i64)
            .collect::<Vec<_>>();

        let result = sqlx::query!("INSERT INTO voice_channels (id, owner_id, trusted_ids, password) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO UPDATE SET owner_id = $2, trusted_ids = $3, password = $4", channel_data.channel_id.get() as i64, channel_data.owner.get() as i64, &trusted_ids, channel_data.password).execute(pool).await?;

        Ok(result.into())
    }
    async fn is_persistent(pool: &Pool<Postgres>, channel_id: ChannelId) -> sqlx::Result<bool> {
        let result = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM voice_channels WHERE id = $1)",
            channel_id.get() as i64
        )
        .fetch_one(pool)
        .await?;

        Ok(result.unwrap_or(false))
    }
}
