use async_trait::async_trait;
use serenity::all::{
    ChannelId, CommandInteraction, Context, CreateCommand, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, EditMessage, Mentionable, MessageId, Permissions, RoleId,
};
use zayden_core::SlashCommand;

use crate::Error;

const LFG_ROLE: RoleId = RoleId::new(944486978693509150);
const DESTINY2_ROLE: RoleId = RoleId::new(931990930385424445);
const FORTNITE_ROLE: RoleId = RoleId::new(1172143188656541746);
const GENSHIN_ROLE: RoleId = RoleId::new(1149110098917064816);
const MINECRAFT_ROLE: RoleId = RoleId::new(1121081685786767440);
const TF2_ROLE: RoleId = RoleId::new(1149110022203265065);

fn message() -> CreateEmbed {
    CreateEmbed::new().title("Game Roles").description(format!("React to this message to get the roles for the games you play!\n\n<:ping:1148819291358105711> ● {}\n\n<:Cayde6ThumbsUp:940629512171970600> ● {}\n<a:fortnite:1172142530914172939> ● {}\n<a:genshin:1150867466805858364> ● {}\n<a:AnimatedMCSheep:1149108483967426560> ● {}\n<:tf2:1149109383180066857> ● {}", LFG_ROLE.mention(), DESTINY2_ROLE.mention(), FORTNITE_ROLE.mention(), GENSHIN_ROLE.mention(), MINECRAFT_ROLE.mention(), TF2_ROLE.mention()))
}

pub struct ReactionRoleMessageCommand;

#[async_trait]
impl SlashCommand<Error> for ReactionRoleMessageCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Error> {
        let channel_id = ChannelId::new(931986133762588752);
        let message_id = MessageId::new(1256772349777285201);

        let _ = channel_id
            .edit_message(ctx, message_id, EditMessage::new().embed(message()))
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
