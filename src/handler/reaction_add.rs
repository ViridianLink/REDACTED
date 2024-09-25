use twilight_model::gateway::GatewayReaction;

use crate::modules::reaction_roles;
use crate::{Client, Result};

pub(super) async fn reaction_add(client: Client, reaction: GatewayReaction) -> Result<()> {
    reaction_roles::reaction::reaction_add(client, &reaction).await?;

    Ok(())
}
