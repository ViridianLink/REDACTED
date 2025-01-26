pub mod destiny2;
pub mod gold_star;
pub mod nsfw;
pub mod reaction_roles;
pub mod temp_voice;

use serenity::all::{Context, CreateCommand, Ready};

use crate::Result;

pub fn global_register(ctx: &Context, ready: &Ready) -> Result<Vec<CreateCommand>> {
    let commands = [
        destiny2::register(ctx, ready)?,
        gold_star::register(ctx, ready)?,
        reaction_roles::register(ctx, ready)?,
        temp_voice::register(ctx, ready)?,
    ]
    .concat();

    Ok(commands)
}
