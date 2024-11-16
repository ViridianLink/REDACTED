use futures::future;
use serenity::all::{Context, OnlineStatus, Ready};
use serenity::all::{CreateCommand, GuildId};
use std::collections::HashMap;

// use crate::cron::start_cron_jobs;
use crate::modules;
// use crate::modules::misc::Sleep;
use crate::Result;

use super::Handler;

impl Handler {
    pub(super) async fn ready(ctx: &Context, ready: Ready) -> Result<()> {
        println!("{} is connected!", ready.user.name);

        ctx.set_presence(None, OnlineStatus::Online);

        // // TODO: Load Commands
        // let mut command_map = guilds::commands();
        let mut command_map: HashMap<GuildId, Vec<CreateCommand>> = HashMap::new();

        let futures = ready.guilds.iter().map(|guild| {
            let mut commands = command_map.remove(&guild.id).unwrap_or_default();
            commands.extend(modules::global_register(ctx, &ready).unwrap());
            guild.id.set_commands(ctx.http.clone(), commands)
        });
        future::try_join_all(futures).await?;

        // let ctx_clone = ctx.clone();
        // tokio::spawn(async move { Sleep::on_ready(ctx_clone, ready).await });

        // let ctx_clone = ctx.clone();
        // tokio::spawn(async move { start_cron_jobs(ctx_clone).await });

        Ok(())
    }
}
// #[async_trait]
// pub trait OnReady {
//     async fn on_ready(ctx: Context, ready: Ready) -> Result<()>;
// }
