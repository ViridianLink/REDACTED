pub mod family;
pub mod gold_star;
pub mod reaction_roles;

use serenity::all::CreateCommand;

pub fn global_register() -> Vec<CreateCommand> {
    [
        family::register(),
        reaction_roles::register(),
        gold_star::register(),
    ]
    .concat()
}
