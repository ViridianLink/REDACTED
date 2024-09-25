use std::sync::Arc;

use twilight_gateway::Event;
use twilight_http::Client as HttpClient;
use twilight_model::application::interaction::{Interaction, InteractionType};

mod interaction_command;
mod interaction_component;
mod reaction_add;
mod reaction_remove;
mod ready;

use interaction_command::interaction_command;
use interaction_component::interaction_component;
use reaction_add::reaction_add;
use reaction_remove::reaction_remove;
use ready::ready;

use crate::{Client, Result, OSCAR_SIX_ID};

pub struct Handler;

#[async_trait]
impl RawEventHandler for Handler {
    async fn raw_event(&self, ctx: Context, ev: Event) {
        let event_name = ev.name().unwrap_or(String::from("Unknown"));
        let ev_command_name = match &ev {
            Event::InteractionCreate(InteractionCreateEvent {
                interaction: Interaction::Command(interaction),
                ..
            }) => interaction.data.name.clone(),
            _ => String::from("Unknown"),
        };
        let ev_debug = format!("{:?}", ev);

        let result = match ev {
            Event::InteractionCreate(interaction) => {
                Self::interaction_create(&ctx, interaction.interaction).await
            }
            Event::ReactionAdd(reaction) => Self::reaction_add(&ctx, reaction.reaction).await,
            Event::ReactionRemove(reaction) => Self::reaction_remove(&ctx, reaction.reaction).await,
            Event::Ready(ready) => Self::ready(&ctx, ready.ready).await,
            _ => Ok(()),
        };

        if let Err(e) = result {
            let msg = format!("Error handling {event_name} | {ev_command_name}: {:?}", e);
            eprintln!("\n{}\n{}\n", msg, ev_debug);

            if let Ok(channel) = OSCAR_SIX_ID.create_dm_channel(&ctx).await {
                let _ = channel.say(&ctx, msg).await;
            }
        }
    }
}

pub async fn handle_event(client: Client, event: Event) {
    let result = match event {
        Event::InteractionCreate(interaction) => interaction_create(client, interaction.0).await,
        Event::ReactionAdd(reaction) => reaction_add(client, reaction.0).await,
        Event::ReactionRemove(reaction) => reaction_remove(client, reaction.0).await,
        Event::Ready(r) => ready(client, r).await,
        _ => Ok(()),

        // Event::MessageCreate(msg) if msg.content == "!ping" => {
        //     http.create_message(msg.channel_id).content("Pong!").await?;
        // }
        _ => Ok(()),
    };
}

async fn interaction_create(client: Client, interaction: Interaction) -> Result<()> {
    match interaction.kind {
        InteractionType::ApplicationCommand => interaction_command(client, interaction).await,
        InteractionType::MessageComponent => interaction_component(client, interaction).await,
        _ => Ok(()),
    }
}
