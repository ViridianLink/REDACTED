pub mod events;
pub mod slash_command;

use async_trait::async_trait;
use serenity::all::{ChannelId, Context, CreateCommand, GuildId, Ready};
use slash_command::VoiceCommand;
use sqlx::{any::AnyQueryResult, PgPool, Postgres};
use temp_voice::voice_channel_manager::VoiceChannelRow;
use temp_voice::{TempVoiceGuildManager, TempVoiceRow, VoiceChannelData, VoiceChannelManager};
use zayden_core::SlashCommand;

use crate::sqlx_lib::GuildTable;
use crate::Result;

pub fn register(ctx: &Context, ready: &Ready) -> Result<Vec<CreateCommand>> {
    let commands = vec![VoiceCommand::register(ctx, ready)?];

    Ok(commands)
}

struct VoiceChannelTable;

#[async_trait]
impl VoiceChannelManager<Postgres> for VoiceChannelTable {
    async fn get(pool: &PgPool, id: ChannelId) -> sqlx::Result<Option<VoiceChannelData>> {
        let row = sqlx::query_as!(
            VoiceChannelRow,
            r#"SELECT * FROM voice_channels WHERE id = $1"#,
            id.get() as i64
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|row| row.into()))
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

#[async_trait]
impl TempVoiceGuildManager<Postgres> for GuildTable {
    async fn save(
        pool: &PgPool,
        id: GuildId,
        category: ChannelId,
        creator_channel: ChannelId,
    ) -> sqlx::Result<AnyQueryResult> {
        let result = sqlx::query!(
            r#"
            INSERT INTO guilds (id, temp_voice_category, temp_voice_creator_channel)
            VALUES ($1, $2, $3)
            ON CONFLICT (id) DO UPDATE
            SET temp_voice_category = $2, temp_voice_creator_channel = $3
            "#,
            id.get() as i64,
            category.get() as i64,
            creator_channel.get() as i64
        )
        .execute(pool)
        .await?;

        Ok(result.into())
    }

    async fn get(pool: &PgPool, id: GuildId) -> sqlx::Result<TempVoiceRow> {
        let row = sqlx::query_as!(
            TempVoiceRow,
            r#"SELECT id, temp_voice_category, temp_voice_creator_channel FROM guilds WHERE id = $1"#,
            id.get() as i64
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    async fn get_category(pool: &PgPool, id: GuildId) -> sqlx::Result<ChannelId> {
        let row = sqlx::query!(
            r#"SELECT temp_voice_category FROM guilds WHERE id = $1"#,
            id.get() as i64
        )
        .fetch_one(pool)
        .await?;

        let category = row
            .temp_voice_category
            .expect("Category ID is required when saving") as u64;

        Ok(ChannelId::from(category))
    }

    async fn get_creator_channel(pool: &PgPool, id: GuildId) -> sqlx::Result<ChannelId> {
        let row = sqlx::query!(
            r#"SELECT temp_voice_creator_channel FROM guilds WHERE id = $1"#,
            id.get() as i64
        )
        .fetch_one(pool)
        .await?;

        let channel_id = row
            .temp_voice_creator_channel
            .expect("Channel ID is required when saving") as u64;

        Ok(ChannelId::from(channel_id))
    }
}
