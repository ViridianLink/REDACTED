use serenity::all::{CommandInteraction, Context, EditInteractionResponse, Mentionable};
use zayden_core::{ErrorResponse, SlashCommand};

use crate::modules::destiny2::dimwishlist::{D2Weapon, DimWishlist};
use crate::modules::destiny2::lfg::LfgCommand;
use crate::modules::family::slash_commands::{
    AdoptCommand, BlockCommand, ChildrenCommand, MarryCommand, ParentsCommand, PartnersCommand,
    RelationshipCommand, SiblingsCommand, TreeCommand, UnblockCommand,
};
use crate::modules::gold_star::slash_commands::{GiveStarCommand, StarsCommand};
use crate::modules::reaction_roles::{ReactionRoleCommand, ReactionRoleMessageCommand};
use crate::modules::temp_voice::VoiceCommand;
use crate::{Error, Result, OSCAR_SIX_ID};

pub async fn interaction_command(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    println!(
        "{} ran command: {}",
        interaction.user.name, interaction.data.name
    );

    let result = match interaction.data.name.as_str() {
        // region Destiny 2
        "d2weapon" => D2Weapon::run(ctx, interaction).await,
        "dimwishlist" => DimWishlist::run(ctx, interaction).await,
        "lfg" => LfgCommand::run(ctx, interaction).await,
        // endregion
        //region Family
        "adopt" => AdoptCommand::run(ctx, interaction).await,
        "block" => BlockCommand::run(ctx, interaction).await,
        "children" => ChildrenCommand::run(ctx, interaction).await,
        "marry" => MarryCommand::run(ctx, interaction).await,
        "parents" => ParentsCommand::run(ctx, interaction).await,
        "partners" => PartnersCommand::run(ctx, interaction).await,
        "relationship" => RelationshipCommand::run(ctx, interaction).await,
        "siblings" => SiblingsCommand::run(ctx, interaction).await,
        "tree" => TreeCommand::run(ctx, interaction).await,
        "unblock" => UnblockCommand::run(ctx, interaction).await,
        // endregion

        // region Gold Stars
        "give_star" => GiveStarCommand::run(ctx, interaction).await,
        "stars" => StarsCommand::run(ctx, interaction).await,
        // endregion

        //region Reaction Roles
        "reaction_role" => ReactionRoleCommand::run(ctx, interaction).await,
        "reaction_role_message" => ReactionRoleMessageCommand::run(ctx, interaction).await,
        //endregion
        // region Temp Voice
        "voice" => VoiceCommand::run(ctx, interaction).await,
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
