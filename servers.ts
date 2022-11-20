import {Server} from "./models/server";
import {ChannelType} from 'discord-api-types/v10';
import {Redacted} from "./client";

export function createServer(guildId: string) {
    const server = new Server({id: guildId})
    server.save().then()
    return server
}

export async function init(client: Redacted) {
    for (const g of await client.guilds.fetch()) {
        const guild = await client.guilds.fetch(g[0])

        let server = await Server.findOne({id: guild.id}).exec() || await createServer(guild.id);

        // Cache reaction messages
        for (let reactionRole of server.reactionRoles) {
            const channel = await client.channels.fetch(reactionRole.channelId)
            if (!channel || channel.type != ChannelType.GuildText) {
                break;
            }

            const msg = await channel.messages.fetch(reactionRole.messageId)
            reactionRole.messageId = msg.id

            const role = await guild.roles.fetch(reactionRole.roleId)
            reactionRole.roleId = role!.id
        }
    }
}
