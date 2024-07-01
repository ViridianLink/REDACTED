use serenity::all::{CommandInteraction, Context, EditInteractionResponse, Mentionable};
use zayden_core::{ErrorResponse, SlashCommand};

use crate::modules::reaction_roles::ReactionRoleCommand;
use crate::modules::reaction_roles::ReactionRoleMessageCommand;
use crate::{Error, Result, OSCAR_SIX_ID};

pub(super) async fn interaction_command(ctx: &Context, command: &CommandInteraction) -> Result<()> {
    println!("{} ran command: {}", command.user.name, command.data.name);

    let result = match command.data.name.as_str() {
        "reaction_role" => ReactionRoleCommand::run(ctx, command).await,
        "reaction_role_message" => ReactionRoleMessageCommand::run(ctx, command).await,
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
