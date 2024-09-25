use futures::future;
use std::collections::HashMap;
use twilight_model::gateway::payload::incoming::Ready;
use twilight_model::gateway::payload::outgoing::UpdatePresence;
use twilight_model::gateway::presence::Status;

// use crate::cron::start_cron_jobs;
use crate::{modules, Client};
// use crate::modules::misc::Sleep;
use crate::Result;

pub(super) async fn ready(client: Client, ready: Ready) -> Result<()> {
    println!("{} is connected!", ready.user.name);

    let sender = client.shard.sender();

    let presence = UpdatePresence::new(Vec::new(), false, None, Status::Online).unwrap();

    sender.command(&presence);

    // // TODO: Load Commands
    // let mut command_map = guilds::commands();
    let mut command_map: HashMap<GuildId, Vec<CreateCommand>> = HashMap::new();

    let futures = ready.guilds.iter().map(|guild| {
        let mut commands = command_map.remove(&guild.id).unwrap_or_default();
        commands.extend(modules::global_register());
        guild.id.set_commands(ctx.http.clone(), commands)
    });
    future::try_join_all(futures).await?;

    // let ctx_clone = ctx.clone();
    // tokio::spawn(async move { Sleep::on_ready(ctx_clone, ready).await });

    // let ctx_clone = ctx.clone();
    // tokio::spawn(async move { start_cron_jobs(ctx_clone).await });

    Ok(())
}
// #[async_trait]
// pub trait OnReady {
//     async fn on_ready(ctx: Context, ready: Ready) -> Result<()>;
// }
