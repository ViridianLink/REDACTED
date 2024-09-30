pub mod events;

use async_trait::async_trait;
use serenity::all::{CommandInteraction, Context, CreateCommand};
use zayden_core::SlashCommand;

use crate::{Error, Result};

pub fn register() -> Vec<CreateCommand> {
    vec![VoiceCommand::register()]
}

pub struct VoiceCommand;

#[async_trait]
impl SlashCommand<Error> for VoiceCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        interaction.defer_ephemeral(ctx).await?;

        temp_voice::VoiceCommand::run(ctx, interaction).await?;

        Ok(())
    }

    fn register() -> CreateCommand {
        temp_voice::VoiceCommand::register()
    }
}
