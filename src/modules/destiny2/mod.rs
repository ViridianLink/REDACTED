use serenity::all::CreateCommand;
use zayden_core::SlashCommand;

mod column;
pub mod manifests;
mod perk;
mod priority;
mod role;
mod shoppinglist;
mod source;
mod weapon_name;
mod weapons;
// pub use manifests::PerkManifest;
pub use shoppinglist::ShoppingList;

pub fn register() -> Vec<CreateCommand> {
    vec![ShoppingList::register()]
}
