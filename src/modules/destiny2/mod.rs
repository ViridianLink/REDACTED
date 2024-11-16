use serenity::all::{Context, CreateCommand, Ready};

pub mod dimwishlist;
pub mod lfg;

use crate::Result;

pub fn register(ctx: &Context, ready: &Ready) -> Result<Vec<CreateCommand>> {
    let commands = [
        dimwishlist::register(ctx, ready)?,
        lfg::register(ctx, ready)?,
    ]
    .concat();

    Ok(commands)
}
