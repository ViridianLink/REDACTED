use serenity::all::{Context, VoiceState};
use sqlx::{PgPool, Postgres};
use temp_voice::events::{channel_creator, channel_deleter};
use temp_voice::VoiceStateCache;

use crate::Result;

use super::VoiceChannelTable;

pub async fn run(ctx: &Context, pool: &PgPool, new: &VoiceState) -> Result<()> {
    let old = VoiceStateCache::update(ctx, new.clone()).await?;

    // Use tokio to run these concurrently
    channel_creator::<Postgres, VoiceChannelTable>(ctx, pool, new).await?;
    channel_deleter::<Postgres, VoiceChannelTable>(ctx, pool, old).await?;

    Ok(())
}
