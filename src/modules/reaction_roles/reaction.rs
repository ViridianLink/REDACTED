use reaction_roles::ReactionRoleReaction;
use sqlx::Postgres;
use twilight_model::gateway::GatewayReaction;

use crate::{Client, Result};

use super::ReactionRolesTable;

pub async fn reaction_add(client: Client, reaction: &GatewayReaction) -> Result<()> {
    ReactionRoleReaction::reaction_add::<Postgres, ReactionRolesTable>(
        client,
        reaction,
        &client.data.pool,
    )
    .await?;

    Ok(())
}

pub async fn reaction_remove(client: Client, reaction: &GatewayReaction) -> Result<()> {
    ReactionRoleReaction::reaction_remove::<Postgres, ReactionRolesTable>(
        client,
        reaction,
        &client.data.pool,
    )
    .await?;

    Ok(())
}
