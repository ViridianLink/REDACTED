use async_trait::async_trait;
use serenity::all::{CommandInteraction, Context, CreateCommand};
use zayden_core::SlashCommand;

use crate::{Error, Result};

pub struct LfgCommand;

#[async_trait]
impl SlashCommand<Error> for LfgCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        interaction.defer_ephemeral(ctx).await?;

        lfg::LfgCommand::run(ctx, interaction).await?;

        Ok(())
    }

    fn register() -> CreateCommand {
        lfg::LfgCommand::register()
    }
}
