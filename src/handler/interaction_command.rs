use serenity::all::{CommandInteraction, Context, EditInteractionResponse, Mentionable};
use zayden_core::{get_option_str, ErrorResponse, SlashCommand};

use crate::modules::destiny2::dimwishlist::{D2Weapon, DimWishlist};
use crate::modules::destiny2::lfg::LfgCommand;
use crate::modules::family::slash_commands::{
    AdoptCommand, BlockCommand, ChildrenCommand, MarryCommand, ParentsCommand, PartnersCommand,
    RelationshipCommand, SiblingsCommand, TreeCommand, UnblockCommand,
};
use crate::modules::gold_star::slash_commands::{GiveStarCommand, StarsCommand};
use crate::modules::reaction_roles::{ReactionRoleCommand, ReactionRoleMessageCommand};
use crate::modules::temp_voice::slash_command::VoiceCommand;
use crate::{Result, OSCAR_SIX_ID};

pub async fn interaction_command(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let options = interaction.data.options();
    let options_str = get_option_str(&options);

    println!(
        "{} ran command: {}{}",
        interaction.user.name, interaction.data.name, options_str
    );

    let result = match interaction.data.name.as_str() {
        // region Destiny 2
        "d2weapon" => D2Weapon::run(ctx, interaction, options).await,
        "dimwishlist" => DimWishlist::run(ctx, interaction, options).await,
        "lfg" => LfgCommand::run(ctx, interaction, options).await,
        // endregion
        //region Family
        "adopt" => AdoptCommand::run(ctx, interaction, options).await,
        "block" => BlockCommand::run(ctx, interaction, options).await,
        "children" => ChildrenCommand::run(ctx, interaction, options).await,
        "marry" => MarryCommand::run(ctx, interaction, options).await,
        "parents" => ParentsCommand::run(ctx, interaction, options).await,
        "partners" => PartnersCommand::run(ctx, interaction, options).await,
        "relationship" => RelationshipCommand::run(ctx, interaction, options).await,
        "siblings" => SiblingsCommand::run(ctx, interaction, options).await,
        "tree" => TreeCommand::run(ctx, interaction, options).await,
        "unblock" => UnblockCommand::run(ctx, interaction, options).await,
        // endregion

        // region Gold Stars
        "give_star" => GiveStarCommand::run(ctx, interaction, options).await,
        "stars" => StarsCommand::run(ctx, interaction, options).await,
        // endregion

        //region Reaction Roles
        "reaction_role" => ReactionRoleCommand::run(ctx, interaction, options).await,
        "reaction_role_message" => ReactionRoleMessageCommand::run(ctx, interaction, options).await,
        //endregion
        // region Temp Voice
        "voice" => VoiceCommand::run(ctx, interaction, options).await,
        // endregion
        _ => {
            println!("Unknown command: {}", interaction.data.name);
            Ok(())
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
                .await
                .unwrap();
            return Err(e);
        }

        interaction
            .edit_response(ctx, EditInteractionResponse::new().content(msg))
            .await
            .unwrap();
    }

    Ok(())
}
