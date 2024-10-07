pub mod family;
pub mod gold_star;
pub mod lfg;
pub mod nsfw;
pub mod reaction_roles;
pub mod temp_voice;

use serenity::all::CreateCommand;

pub fn global_register() -> Vec<CreateCommand> {
    [
        family::register(),
        reaction_roles::register(),
        gold_star::register(),
        temp_voice::register(),
    ]
    .concat()
}
