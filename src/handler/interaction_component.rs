use serenity::all::{ComponentInteraction, CreateInteractionResponseFollowup};
use serenity::all::{Context, Mentionable};
use zayden_core::ErrorResponse;

use crate::modules::family::components::{AdoptComponent, MarryComponent};
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
        "lfg_join" => lfg::PostComponents::join(ctx, component)
            .await
            .map_err(Error::from),
        "lfg_leave" => lfg::PostComponents::leave(ctx, component)
            .await
            .map_err(Error::from),
        // endregion
        _ => Err(Error::UnknownComponent(component.data.custom_id.clone())),
    };

    if let Err(e) = result {
        let msg = e.to_response();
        if msg.is_empty() {
            component
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
        component
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
