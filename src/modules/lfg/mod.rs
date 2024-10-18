mod components;
mod modal;
mod slash_command;

use async_trait::async_trait;
use chrono::{DateTime, FixedOffset};
use lfg::{LfgPostManager, LfgPostRow};
use serenity::all::{CreateCommand, MessageId};
use sqlx::{any::AnyQueryResult, Pool, Postgres};
use zayden_core::SlashCommand;

pub use components::LfgComponents;
pub use modal::LfgCreateModal;
pub use slash_command::LfgCommand;

pub fn register() -> Vec<CreateCommand> {
    vec![LfgCommand::register()]
}

struct LfgPostTable;

#[async_trait]
impl LfgPostManager<Postgres> for LfgPostTable {
    async fn get(
        pool: &Pool<Postgres>,
        id: impl Into<MessageId> + Send,
    ) -> sqlx::Result<LfgPostRow> {
        let post = sqlx::query_as!(
            LfgPostRow,
            "SELECT * FROM lfg_posts WHERE id = $1",
            id.into().get() as i64
        )
        .fetch_one(pool)
        .await?;

        Ok(post)
    }

    async fn save(
        pool: &Pool<Postgres>,
        id: impl Into<i64> + Send,
        owner: impl Into<i64> + Send,
        activity: &str,
        start_time: DateTime<FixedOffset>,
        description: &str,
        fireteam_size: impl Into<i16> + Send,
        fireteam_ids: &[i64],
    ) -> sqlx::Result<AnyQueryResult> {
        let result = sqlx::query!(
            "INSERT INTO lfg_posts (id, owner_id, activity, start_time, description, fireteam_size, fireteam_ids)
            VALUES ($7, $1, $2, $3, $4, $5, $6)
            ON CONFLICT (id)
            DO UPDATE SET owner_id = EXCLUDED.owner_id,
                          activity = EXCLUDED.activity,
                          start_time = EXCLUDED.start_time,
                          description = EXCLUDED.description,
                          fireteam_size = EXCLUDED.fireteam_size,
                          fireteam_ids = EXCLUDED.fireteam_ids;",
            owner.into(),
            activity,
            start_time,
            description,
            fireteam_size.into(),
            fireteam_ids,
            id.into()
        )
        .execute(pool)
        .await?;

        Ok(result.into())
    }
}
