use dimwishlist::DimWishlist;
use lfg::LfgCommand;
use serenity::all::{Context, CreateCommand, Ready};
use tierlist::TierList;
use weapon::WeaponCommand;
use zayden_core::SlashCommand;

pub mod dimwishlist;
pub mod endgame_analysis;
pub mod lfg;
pub mod tierlist;
pub mod weapon;

use crate::Result;

pub fn register(ctx: &Context, ready: &Ready) -> Result<Vec<CreateCommand>> {
    let commands = vec![
        DimWishlist::register(ctx, ready)?,
        LfgCommand::register(ctx, ready)?,
        WeaponCommand::register(ctx, ready)?,
        TierList::register(ctx, ready)?,
    ];

    Ok(commands)
}
