// pub struct ReactionRoleMessageCommand;

// #[async_trait]
// impl SlashCommand<Error> for ReactionRoleMessageCommand {
//     async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Error> {
//         let _ = CHANNEL_ID
//             .send_message(ctx, CreateMessage::new().embed(message()))
//             .await;

//         interaction
//             .create_response(
//                 ctx,
//                 CreateInteractionResponse::Message(
//                     CreateInteractionResponseMessage::new()
//                         .ephemeral(true)
//                         .content("Message updated."),
//                 ),
//             )
//             .await?;

//         Ok(())
//     }

//     fn register() -> CreateCommand {
//         CreateCommand::new("reaction_role_message")
//             .description("Edit the reaction role message.")
//             .default_member_permissions(Permissions::ADMINISTRATOR)
//     }
// }
