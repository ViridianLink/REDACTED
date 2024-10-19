use serenity::all::{
    ComponentInteraction, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::all::{Context, Mentionable};
use zayden_core::ErrorResponse;

use crate::modules::family::components::{AdoptComponent, MarryComponent};
use crate::modules::lfg::LfgComponents;
use crate::{Error, Result, OSCAR_SIX_ID};

pub(super) async fn interaction_component(
    ctx: &Context,
    component: &ComponentInteraction,
) -> Result<()> {
    println!(
        "{} ran component: {}",
        component.user.name, component.data.custom_id
    );

    let result = match component.data.custom_id.as_str() {
        //region Family
        "adopt_accept" => AdoptComponent::accept(ctx, component).await,
        "adopt_decline" => AdoptComponent::decline(ctx, component).await,

        "marry_accept" => MarryComponent::accept(ctx, component).await,
        "marry_decline" => MarryComponent::decline(ctx, component).await,
        //endregion

        // region LFG
        "lfg_activity" => lfg::ActivityComponent::run(ctx, component)
            .await
            .map_err(Error::from),
        "lfg_join" => LfgComponents::join(ctx, component).await,
        "lfg_leave" => LfgComponents::leave(ctx, component).await,
        "lfg_alternative" => LfgComponents::alternative(ctx, component).await,
        "lfg_settings" => LfgComponents::settings(ctx, component).await,

        "lfg_edit" => LfgComponents::edit(ctx, component).await,
        "lfg_copy" => LfgComponents::copy(ctx, component).await,
        "lfg_kick" => LfgComponents::kick(ctx, component).await,
        "lfg_kick_menu" => LfgComponents::kick_menu(ctx, component).await,
        "lfg_delete" => LfgComponents::delete(ctx, component).await,
        // endregion
        _ => Err(Error::UnknownComponent(component.data.custom_id.clone())),
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

        component
            .create_response(
                ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(content)
                        .ephemeral(true),
                ),
            )
            .await?;

        if is_error {
            return Err(e);
        }
    }
    Ok(())
}
