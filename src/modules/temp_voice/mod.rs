pub mod events;

use async_trait::async_trait;
use serenity::all::{ChannelId, CommandInteraction, Context, CreateCommand, Ready};
use sqlx::{any::AnyQueryResult, PgPool, Postgres};
use temp_voice::voice_channel_manager::VoiceChannelRow;
use temp_voice::{VoiceChannelData, VoiceChannelManager};
use zayden_core::SlashCommand;

use crate::{sqlx_lib::PostgresPool, Error, Result};

pub fn register(ctx: &Context, ready: &Ready) -> Result<Vec<CreateCommand>> {
    let commands = vec![VoiceCommand::register(ctx, ready)?];

    Ok(commands)
}

pub struct VoiceCommand;

#[async_trait]
impl SlashCommand<Error> for VoiceCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        interaction.defer_ephemeral(ctx).await?;

        temp_voice::VoiceCommand::run::<Postgres, VoiceChannelTable>(ctx, interaction, &pool)
            .await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(temp_voice::VoiceCommand::register())
    }
}

struct VoiceChannelTable;

#[async_trait]
impl VoiceChannelManager<Postgres> for VoiceChannelTable {
    async fn get(pool: &PgPool, id: ChannelId) -> sqlx::Result<VoiceChannelData> {
        let row = sqlx::query_as!(
            VoiceChannelRow,
            r#"SELECT * FROM voice_channels WHERE id = $1"#,
            id.get() as i64
        )
        .fetch_one(pool)
        .await?;

        Ok(row.into())
    }

    async fn save(
        pool: &PgPool,
        id: impl Into<i64> + Send,
        owner_id: impl Into<i64> + Send,
        trusted_ids: &[i64],
        password: Option<&str>,
        persistent: impl Into<bool> + Send,
    ) -> sqlx::Result<AnyQueryResult> {
        let result = sqlx::query!(
            r#"
            INSERT INTO voice_channels (id, owner_id, trusted_ids, password, persistent)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE
            SET owner_id = $2, trusted_ids = $3, password = $4, persistent = $5
            "#,
            id.into(),
            owner_id.into(),
            trusted_ids,
            password,
            persistent.into()
        )
        .execute(pool)
        .await?;

        Ok(result.into())
    }

    async fn delete(pool: &PgPool, id: ChannelId) -> sqlx::Result<AnyQueryResult> {
        let result = sqlx::query!(
            r#"DELETE FROM voice_channels WHERE id = $1"#,
            id.get() as i64
        )
        .execute(pool)
        .await?;

        Ok(result.into())
    }
}
