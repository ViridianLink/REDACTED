use serenity::all::{CommandInteraction, Context, EditInteractionResponse};
use sqlx::PgPool;
use zayden_core::{Autocomplete, ErrorResponse};

use crate::Result;
use crate::modules::destiny2::lfg::LfgCommand;
use crate::modules::destiny2::tierlist::TierList;
use crate::modules::destiny2::weapon::WeaponCommand;

pub async fn interaction_autocomplete(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &PgPool,
) -> Result<()> {
    let option = interaction.data.autocomplete().unwrap();

    let result = match interaction.data.name.as_str() {
        // region Destiny 2
        "lfg" => LfgCommand::autocomplete(ctx, interaction, option, pool).await,
        "weapon" => WeaponCommand::autocomplete(ctx, interaction, option, pool).await,
        "tierlist" => TierList::autocomplete(ctx, interaction, option, pool).await,
        // endregion
        _ => {
            println!("Unknown autocomplete: {}", interaction.data.name);
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
