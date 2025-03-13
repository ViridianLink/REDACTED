use serenity::all::{CommandInteraction, Context, EditInteractionResponse};
use sqlx::PgPool;
use zayden_core::{ErrorResponse, SlashCommand, get_option_str};

use crate::Result;
use crate::modules::destiny2::dimwishlist::DimWishlist;
use crate::modules::destiny2::lfg::LfgCommand;
use crate::modules::destiny2::tierlist::TierList;
use crate::modules::destiny2::weapon::WeaponCommand;
use crate::modules::gold_star::slash_commands::{GiveStarCommand, StarsCommand};
use crate::modules::temp_voice::slash_command::Voice;

pub async fn interaction_command(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &PgPool,
) -> Result<()> {
    let options = interaction.data.options();
    let options_str = get_option_str(&options);

    println!(
        "{} ran command: {}{}",
        interaction.user.name, interaction.data.name, options_str
    );

    let result = match interaction.data.name.as_str() {
        // region Destiny 2
        "weapon" => WeaponCommand::run(ctx, interaction, options, pool).await,
        "dimwishlist" => DimWishlist::run(ctx, interaction, options, pool).await,
        "lfg" => LfgCommand::run(ctx, interaction, options, pool).await,
        "tierlist" => TierList::run(ctx, interaction, options, pool).await,
        // endregion

        // region Gold Stars
        "give_star" => GiveStarCommand::run(ctx, interaction, options, pool).await,
        "stars" => StarsCommand::run(ctx, interaction, options, pool).await,
        // endregion

        // region Temp Voice
        "voice" => Voice::run(ctx, interaction, options, pool).await,
        // endregion
        _ => {
            println!("Unknown command: {}", interaction.data.name);
            Ok(())
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
