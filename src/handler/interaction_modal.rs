use serenity::all::{Context, CreateInteractionResponseFollowup, Mentionable, ModalInteraction};
use zayden_core::ErrorResponse;

use crate::modules::destiny2::lfg;
use crate::{Result, OSCAR_SIX_ID};

pub async fn interaction_modal(ctx: &Context, interaction: &ModalInteraction) -> Result<()> {
    println!(
        "{} ran modal: {}",
        interaction.user.name, interaction.data.custom_id
    );

    let result = match interaction.data.custom_id.as_str() {
        // region LFG
        "lfg_create" => lfg::LfgCreateModal::run(ctx, interaction).await,
        "lfg_edit" => lfg::LfgEditModal::run(ctx, interaction).await,
        // endregion
        _ => {
            println!("Unknown modal: {}", interaction.data.custom_id);
            return Ok(());
        }
    };

    if let Err(e) = result {
        let msg = e.to_response();
        if msg.is_empty() {
            interaction
                .create_followup(
                    ctx,
                    CreateInteractionResponseFollowup::new().content(format!(
                        "An error occurred. Please contact {} if this issue persists.",
                        OSCAR_SIX_ID.mention()
                    )),
                )
                .await
                .unwrap();
            return Err(e);
        }
        interaction
            .create_followup(
                ctx,
                CreateInteractionResponseFollowup::new()
                    .content(msg)
                    .ephemeral(true),
            )
            .await
            .unwrap();
    }

    Ok(())
}
