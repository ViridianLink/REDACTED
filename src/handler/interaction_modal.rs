use serenity::all::{Context, CreateInteractionResponseFollowup, Mentionable, ModalInteraction};
use zayden_core::ErrorResponse;

use crate::modules::lfg;
use crate::{Error, Result, OSCAR_SIX_ID};

pub(super) async fn interaction_modal(ctx: &Context, modal: &ModalInteraction) -> Result<()> {
    println!("{} ran modal: {}", modal.user.name, modal.data.custom_id);

    let result = match modal.data.custom_id.as_str() {
        // region LFG
        "lfg_create" => lfg::LfgCreateModal::run(ctx, modal).await,
        "lfg_edit" => lfg::LfgEditModal::run(ctx, modal).await,
        // endregion
        _ => Err(Error::UnknownComponent(modal.data.custom_id.clone())),
    };

    if let Err(e) = result {
        let msg = e.to_response();
        if msg.is_empty() {
            modal
                .create_followup(
                    ctx,
                    CreateInteractionResponseFollowup::new().content(format!(
                        "An error occurred. Please contact {} if this issue persists.",
                        OSCAR_SIX_ID.mention()
                    )),
                )
                .await?;
            return Err(e);
        }
        modal
            .create_followup(
                ctx,
                CreateInteractionResponseFollowup::new()
                    .content(msg)
                    .ephemeral(true),
            )
            .await?;
    }

    Ok(())
}
