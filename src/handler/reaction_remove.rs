use serenity::all::{Context, Reaction};

use crate::modules::reaction_roles;
use crate::Result;

use super::Handler;

impl Handler {
    pub(super) async fn reaction_remove(ctx: &Context, reaction: Reaction) -> Result<()> {
        reaction_roles::reaction::reaction_remove(ctx, &reaction).await?;

        Ok(())
    }
}
