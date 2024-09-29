use serenity::all::{Context, VoiceState};

use crate::modules::temp_voice;
use crate::Result;

use super::Handler;

impl Handler {
    pub(super) async fn voice_state_update(ctx: &Context, new: VoiceState) -> Result<()> {
        temp_voice::events::run(ctx, &new).await?;

        Ok(())
    }
}
