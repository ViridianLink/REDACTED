use serenity::all::{Context, EditInteractionResponse, ModalInteraction};
use zayden_core::ErrorResponse;

use crate::modules::destiny2::lfg;
use crate::Result;

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

        let _ = interaction.defer_ephemeral(ctx).await;

        interaction
            .edit_response(ctx, EditInteractionResponse::new().content(msg))
            .await
            .unwrap();
    }

    Ok(())
}
