use serenity::all::{CommandInteraction, Context, EditInteractionResponse, Mentionable};
use zayden_core::{Autocomplete, ErrorResponse};

use crate::modules::destiny2::dimwishlist::D2Weapon;
use crate::modules::destiny2::lfg::LfgCommand;
use crate::{Error, Result, OSCAR_SIX_ID};

pub async fn interaction_autocomplete(
    ctx: &Context,
    interaction: &CommandInteraction,
) -> Result<()> {
    let result = match interaction.data.name.as_str() {
        // region Destiny 2
        "lfg" => LfgCommand::autocomplete(ctx, interaction).await,
        "d2weapon" => D2Weapon::autocomplete(ctx, interaction).await,
        // endregion
        _ => {
            return Err(Error::UnknownCommand(interaction.data.name.clone()));
        }
    };

    if let Err(e) = result {
        let _ = interaction.defer(ctx).await;

        let msg = e.to_response();
        if msg.is_empty() {
            interaction
                .edit_response(
                    ctx,
                    EditInteractionResponse::new().content(format!(
                        "An error occurred. Please contact {} if this issue persists.",
                        OSCAR_SIX_ID.mention()
                    )),
                )
                .await?;
            return Err(e);
        }

        interaction
            .edit_response(ctx, EditInteractionResponse::new().content(msg))
            .await?;
    }

    Ok(())
}
