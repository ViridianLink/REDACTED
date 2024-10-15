use serenity::all::{Context, CreateInteractionResponse, ModalInteraction};

use crate::Result;

pub struct LfgCreateModal;

impl LfgCreateModal {
    pub async fn run(ctx: &Context, interaction: &ModalInteraction) -> Result<()> {
        lfg::LfgCreateModal::run(ctx, interaction).await?;

        interaction
            .create_response(ctx, CreateInteractionResponse::Acknowledge)
            .await?;

        Ok(())
    }
}
