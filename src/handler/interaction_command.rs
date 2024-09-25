use twilight_model::application::interaction::Interaction;
use zayden_core::{ErrorResponse, SlashCommand};

use crate::modules::family::slash_commands::{
    AdoptCommand, BlockCommand, ChildrenCommand, MarryCommand, ParentsCommand, PartnersCommand,
    RelationshipCommand, SiblingsCommand, TreeCommand, UnblockCommand,
};
use crate::modules::gold_star::slash_commands::{GiveStarCommand, StarsCommand};
use crate::modules::reaction_roles::{ReactionRoleCommand, ReactionRoleMessageCommand};
use crate::{Client, Error, Result, OSCAR_SIX_ID};

pub(super) async fn interaction_command(client: Client, command: Interaction) -> Result<()> {
    println!("{} ran command: {}", command.user.name, command.data.name);

    let result = match command.data.name.as_str() {
        //region Family
        "adopt" => AdoptCommand::run(ctx, command).await,
        "block" => BlockCommand::run(ctx, command).await,
        "children" => ChildrenCommand::run(ctx, command).await,
        "marry" => MarryCommand::run(ctx, command).await,
        "parents" => ParentsCommand::run(ctx, command).await,
        "partners" => PartnersCommand::run(ctx, command).await,
        "relationship" => RelationshipCommand::run(ctx, command).await,
        "siblings" => SiblingsCommand::run(ctx, command).await,
        "tree" => TreeCommand::run(ctx, command).await,
        "unblock" => UnblockCommand::run(ctx, command).await,
        // endregion

        // region Gold Stars
        "give_star" => GiveStarCommand::run(ctx, command).await,
        "stars" => StarsCommand::run(ctx, command).await,
        // endregion

        //region Reaction Roles
        "reaction_role" => ReactionRoleCommand::run(ctx, command).await,
        "reaction_role_message" => ReactionRoleMessageCommand::run(ctx, command).await,
        //endregion
        _ => Err(Error::UnknownCommand(command.data.name.clone())),
    };

    if let Err(e) = result {
        let _ = command.defer(ctx).await;

        let msg = e.to_response();
        if msg.is_empty() {
            command
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

        command
            .edit_response(ctx, EditInteractionResponse::new().content(msg))
            .await?;
    }

    Ok(())
}
