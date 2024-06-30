use serenity::all::{Context, Reaction};

use crate::modules::reaction_roles;
use crate::Result;

use super::Handler;

impl Handler {
    pub(super) async fn reaction_add(ctx: &Context, reaction: Reaction) -> Result<()> {
        reaction_roles::reaction::reaction_add(ctx, &reaction).await?;

        Ok(())
    }
}
