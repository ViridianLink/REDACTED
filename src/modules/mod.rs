use async_trait::async_trait;
use serenity::all::{CommandInteraction, Context, CreateCommand};

use crate::Result;



pub fn global_register() -> Vec<CreateCommand> {
    [
        // family::register(),
        // gold_star::register(),
        // misc::register(),
        // moderation::register(),
    ]
    .concat()
}