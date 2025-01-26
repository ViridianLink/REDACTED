use serenity::all::Context;
use serenity::all::{ComponentInteraction, EditInteractionResponse};
use zayden_core::ErrorResponse;

use crate::modules::destiny2::lfg::LfgComponents;
use crate::Result;

pub async fn interaction_component(
    ctx: &Context,
    interaction: &ComponentInteraction,
) -> Result<()> {
    println!(
        "{} ran component: {} - {}",
        interaction.user.name, interaction.data.custom_id, interaction.message.id
    );

    let result = match interaction.data.custom_id.as_str() {
        // region LFG
        "lfg_join" => LfgComponents::join(ctx, interaction).await,
        "lfg_leave" => LfgComponents::leave(ctx, interaction).await,
        "lfg_alternative" => LfgComponents::alternative(ctx, interaction).await,
        "lfg_settings" => LfgComponents::settings(ctx, interaction).await,

        "lfg_edit" => LfgComponents::edit(ctx, interaction).await,
        "lfg_copy" => LfgComponents::copy(ctx, interaction).await,
        "lfg_kick" => LfgComponents::kick(ctx, interaction).await,
        "lfg_kick_menu" => LfgComponents::kick_menu(ctx, interaction).await,
        "lfg_delete" => LfgComponents::delete(ctx, interaction).await,
        // endregion
        _ => {
            println!("Unknown component: {}", interaction.data.custom_id);
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
