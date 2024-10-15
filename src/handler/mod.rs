// mod interaction_create;
// mod message;
// mod reaction;
mod interaction_command;
mod interaction_component;
mod interaction_modal;
mod reaction_add;
mod reaction_remove;
mod ready;
mod voice_state_update;

use serenity::all::{Event, InteractionCreateEvent, RawEventHandler};
use serenity::async_trait;
use serenity::model::prelude::Interaction;
use serenity::prelude::Context;

use crate::{Result, OSCAR_SIX_ID};
use interaction_command::interaction_command;
use interaction_component::interaction_component;
use interaction_modal::interaction_modal;

// pub use ready::OnReady;

pub struct Handler;

impl Handler {
    async fn interaction_create(ctx: &Context, interaction: Interaction) -> Result<()> {
        match &interaction {
            Interaction::Command(command) => interaction_command(ctx, command).await?,
            Interaction::Component(component) => interaction_component(ctx, component).await?,
            Interaction::Modal(modal) => interaction_modal(ctx, modal).await?,
            _ => unimplemented!("Interaction not implemented: {:?}", interaction.kind()),
        };

        Ok(())
    }
}

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
            Event::VoiceStateUpdate(voice_state) => {
                Self::voice_state_update(&ctx, voice_state.voice_state).await
            }

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
