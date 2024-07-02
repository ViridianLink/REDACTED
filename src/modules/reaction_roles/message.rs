use async_trait::async_trait;
use serenity::all::{
    ChannelId, CommandInteraction, Context, CreateCommand, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, Mentionable, Permissions, RoleId,
};
use zayden_core::SlashCommand;

use crate::Error;

const RED_ROLE: RoleId = RoleId::new(923681972600045618);
const PURPLE_ROLE: RoleId = RoleId::new(932007593738645584);
const BLUE_ROLE: RoleId = RoleId::new(932007591943491594);
const GREEN_ROLE: RoleId = RoleId::new(932007590395797535);
const YELLOW_ROLE: RoleId = RoleId::new(932007475144720424);
const ORANGE_ROLE: RoleId = RoleId::new(932007389937422408);
const BROWN_ROLE: RoleId = RoleId::new(1257494437370794055);
const WHITE_ROLE: RoleId = RoleId::new(932007595303112714);
const BLACK_ROLE: RoleId = RoleId::new(932007597006024745);

const CHANNEL_ID: ChannelId = ChannelId::new(932004859899691018);
// const MESSAGE_ID: MessageId = MessageId::new(1256772349777285201);

fn message() -> CreateEmbed {
    CreateEmbed::new()
        .title("Colour Roles")
        .description(format!(
            "React to this message to get a colour role.\n\n🔴 ● {}\n🟣 ● {}\n🔵 ● {}\n🟢 ● {}\n🟡 ● {}\n🟠 ● {}\n🟤 ● {}\n⚪ ● {}\n⚫ ● {}", RED_ROLE.mention(), PURPLE_ROLE.mention(), BLUE_ROLE.mention(), GREEN_ROLE.mention(), YELLOW_ROLE.mention(), ORANGE_ROLE.mention(), BROWN_ROLE.mention(), WHITE_ROLE.mention(), BLACK_ROLE.mention()
        ))
}

pub struct ReactionRoleMessageCommand;

#[async_trait]
impl SlashCommand<Error> for ReactionRoleMessageCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Error> {
        let _ = CHANNEL_ID
            .send_message(ctx, CreateMessage::new().embed(message()))
            .await;

        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .ephemeral(true)
                        .content("Message updated."),
                ),
            )
            .await?;

        Ok(())
    }

    fn register() -> CreateCommand {
        CreateCommand::new("reaction_role_message")
            .description("Edit the reaction role message.")
            .default_member_permissions(Permissions::ADMINISTRATOR)
    }
}
