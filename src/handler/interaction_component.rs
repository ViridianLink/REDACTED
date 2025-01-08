use serenity::all::{
    ComponentInteraction, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::all::{Context, Mentionable};
use zayden_core::ErrorResponse;

use crate::modules::destiny2::lfg::LfgComponents;
use crate::modules::family::components::{AdoptComponent, MarryComponent};
use crate::{Result, OSCAR_SIX_ID};

pub async fn interaction_component(
    ctx: &Context,
    interaction: &ComponentInteraction,
) -> Result<()> {
    println!(
        "{} ran component: {} - {}",
        interaction.user.name, interaction.data.custom_id, interaction.message.id
    );

    let result = match interaction.data.custom_id.as_str() {
        //region Family
        "adopt_accept" => AdoptComponent::accept(ctx, interaction).await,
        "adopt_decline" => AdoptComponent::decline(ctx, interaction).await,

        "marry_accept" => MarryComponent::accept(ctx, interaction).await,
        "marry_decline" => MarryComponent::decline(ctx, interaction).await,
        //endregion

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
        let is_error = msg.is_empty();

        let content = match is_error {
            true => format!(
                "An error occurred. Please contact {} if this issue persists.",
                OSCAR_SIX_ID.mention()
            ),
            false => msg,
        };

        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(content)
                        .ephemeral(true),
                ),
            )
            .await
            .unwrap();

        if is_error {
            return Err(e);
        }
    }
    Ok(())
}
