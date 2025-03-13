use serenity::all::{CommandInteraction, Context, EditInteractionResponse};
use sqlx::PgPool;
use zayden_core::{get_option_str, ErrorResponse, SlashCommand};

use crate::modules::destiny2::dimwishlist::{D2Weapon, DimWishlist};
use crate::modules::destiny2::lfg::LfgCommand;
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
        "d2weapon" => D2Weapon::run(ctx, interaction, options, pool).await,
        "dimwishlist" => DimWishlist::run(ctx, interaction, options, pool).await,
        "lfg" => LfgCommand::run(ctx, interaction, options, pool).await,
        // endregion

        // region Gold Stars
        "give_star" => GiveStarCommand::run(ctx, interaction, options, pool).await,
        "stars" => StarsCommand::run(ctx, interaction, options, pool).await,
        // endregion

        // region Temp Voice
        "voice" => VoiceCommand::run(ctx, interaction, options, pool).await,
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
