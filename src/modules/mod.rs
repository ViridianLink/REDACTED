pub mod reaction_roles;

use serenity::all::CreateCommand;

pub fn global_register() -> Vec<CreateCommand> {
    [reaction_roles::register()].concat()
}
