mod modal;
mod slash_command;

use serenity::all::CreateCommand;
use zayden_core::SlashCommand;

pub use modal::LfgCreateModal;
pub use slash_command::LfgCommand;

pub fn register() -> Vec<CreateCommand> {
    vec![LfgCommand::register()]
}
