use serenity::all::{Context, VoiceState};
use temp_voice::events::{channel_creator, channel_deleter};
use temp_voice::VoiceStateCache;

use crate::Result;

pub async fn run(ctx: &Context, new: &VoiceState) -> Result<()> {
    let old = VoiceStateCache::update(ctx, new.clone()).await?;

    // Use tokio to run these concurrently
    channel_creator(ctx, new).await?;
    channel_deleter(ctx, old).await?;

    Ok(())
}
