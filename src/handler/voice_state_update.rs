use serenity::all::{Context, VoiceState};

use crate::modules::temp_voice;
use crate::sqlx_lib::PostgresPool;
use crate::Result;

use super::Handler;

impl Handler {
    pub(super) async fn voice_state_update(ctx: &Context, new: VoiceState) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        temp_voice::events::run(ctx, &pool, &new).await?;

        Ok(())
    }
}
