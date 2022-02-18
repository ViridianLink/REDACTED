import Discord from "discord.js";
import dotenv from "dotenv";
import fs from "fs";
import path from "path";
import { ICommand } from "./commands/command";
import { commands } from "./commands/load_commands";
import { Server, servers } from "./server";


dotenv.config()


export const client = new Discord.Client({
    intents: [
        Discord.Intents.FLAGS.GUILDS,
        Discord.Intents.FLAGS.GUILD_MEMBERS,
        Discord.Intents.FLAGS.GUILD_MESSAGES,
        Discord.Intents.FLAGS.GUILD_MESSAGE_REACTIONS,
        Discord.Intents.FLAGS.GUILD_VOICE_STATES
    ],
    partials: ['MESSAGE', 'CHANNEL', 'REACTION']
})

// Initialize database
require("./sql").init()


// Init
client.on("ready", () => {
    const botConfig = require("./configs/bot_config.json");
    console.log(`REDACTED is Running, version: ${botConfig.version}`);
    require("./commands/deploy_commands").deployCommands(client)

    if (client.user) {
        client.user.setPresence({ activities: [{ name: "Destiny 2" }], status: "online" })
    }

    // Load server configs
    const serverConfigFiles = fs.readdirSync(path.join(__dirname, "server_configs"))
    for (const filename of serverConfigFiles) {
        const serverConfig = require(path.join(__dirname, "server_configs", filename))
        const guildId = path.parse(filename).name

        const server = new Server(guildId)
        server.reactionRoles = serverConfig.reactionRoles
        server.disabledCommands = serverConfig.disabledCommands
        server.roles = serverConfig.roles
        server.channels = serverConfig.channels
        server.serverRules = serverConfig.serverRules
        server.hidden = serverConfig.hidden
        server.moderation = serverConfig.moderation

        servers[guildId] = server
    }

    for (const guild of client.guilds.cache.values()) {
        if (!(guild.id in servers)) {
            const server = new Server(guild.id)
            servers[guild.id] = server

            fs.writeFile(`./server_configs/${guild.id}.json`, JSON.stringify(server, null, 4), function writeJSON(err) {
                if (err) { return console.log(err); }
            });
        }

        const server = servers[guild.id]

        // Cache reaction messages
        for (let reactionRole of server.reactionRoles) {
            const channel = client.channels.cache.get(reactionRole.channelId) as Discord.TextChannel
            if (!channel) { break; }
            reactionRole.channelId = channel.id;

            channel.messages.fetch(reactionRole.messageId)
                .then((msg: Discord.Message) => { reactionRole.messageId = msg.id })

            guild.roles.fetch(reactionRole.roleId)
                .then(role => { if (role) { reactionRole.roleId = role.id; } })
        }

        if (server.roles.default) {
            let defaultRoles: Discord.RoleResolvable[] = []
            server.roles.default.forEach(roleId => {
                guild.roles.fetch(roleId).then((role) => {
                    if (!role) return;
                    defaultRoles.push(role)
                })
            })

            // Add default roles to every member
            guild.members.fetch().then((members) => {
                for (let member of members.values()) {
                    member.roles.add(defaultRoles)
                }
            })
        }
    }

    // init.updateImages();

    const blacklist = require("./blacklist")
    blacklist.init()

    // const moderation = require("./moderationFunctions")
    // moderation.init()

    // Self Updating
    // const update_guidelines = require("./self_updating/updateGuidelines");
    // update_guidelines(client, "879894434538459157")

    const customRoles = require("./self_updating/customRoles")
    customRoles.gameRoles(client, "931986133762588752", "931988680057454592")
    customRoles.colourRoles(client, "932004859899691018", "932004884839039047")

    // const updateInfomation = require("./self_updating/updateInfomation")
    // updateInfomation(client, "830927865784565800")

    // const updateRules = require("./self_updating/updateRules")
    // updateRules(client, "747430712617074718")
});


client.on("guildCreate", async guild => {
    const server = new Server(guild.id)

    fs.writeFile(`./server_configs/${guild.id}.json`, JSON.stringify(server, null, 4), function writeJSON(err) {
        if (err) { return console.log(err); }
    });
})


client.on("guildDelete", async guild => {
    fs.rename(`./server_configs/${guild.id}.json`, `./server_configs/OLD_${guild.id}.json`, (error) => {
        if (error) { console.log(error) }
    })
})


client.on("guildMemberAdd", async (member) => {
    const guild = member.guild
    const server = servers[guild.id]

    const common = require("./common")
    const defaultRoles = await common.getRoleResolveables(guild, server.roles.default)

    member.roles.add(defaultRoles)
})


client.on("interactionCreate", async interaction => {
    if (!interaction.isCommand()) return;

    const command: ICommand | undefined = commands.get(interaction.commandName);
    if (!command) { return; }

    await command.callback(interaction, interaction.options, null)
});


client.on("messageCreate", message => {
    require("./commands/command_base").runMessageCommand(message)

    require("./special_commands/questionMe")(message)
})


client.on("messageReactionAdd", async (reaction, user) => {
    const guild = reaction.message.guild
    if (!guild) return;

    const server = servers[guild.id];

    for (const reactionRole of server.reactionRoles) {
        if (reaction.message.id == reactionRole.messageId && reaction.emoji.toString() == reactionRole.emoji && user.id !== client.user?.id) {
            const member = guild.members.cache.find(member => member.id == user.id)
            if (!member) { break; }

            const role = await guild.roles.fetch(reactionRole.roleId)
            if (!role) { break; }

            member.roles.add(role)
                .catch((error) => console.log(error))
            break;
        }
    }
})


client.on("messageReactionRemove", async (reaction, user) => {
    const guild = reaction.message.guild
    if (!guild) return;

    const server = servers[guild.id];

    for (const reactionRole of server.reactionRoles) {
        if (reaction.message.id == reactionRole.messageId && reaction.emoji.toString() == reactionRole.emoji && user.id !== client.user?.id) {
            const member = guild.members.cache.find(member => member.id == user.id)
            if (!member) { break; }

            const role = await guild.roles.fetch(reactionRole.roleId)
            if (!role) { break; }

            member.roles.remove(role)
                .catch((error) => console.log(error))
            break;
        }
    }
})


// Create new channel when user joins empty channel
client.on("voiceStateUpdate", async (oldState, newState) => {
    const guild = oldState.guild;
    const newChannel = newState.channel

    if (!newChannel || newChannel.parentId != "923679215205892098" || newChannel.members.size > 1) { return; }

    guild.channels.fetch("923679215205892098") // Room's Catagory
        .then((catagory) => {
            if (!catagory || catagory.type != "GUILD_CATEGORY") return;

            const lastChannel = catagory.children.sort((first, second) => { return first.name.localeCompare(second.name); }).last()
            if (!lastChannel || lastChannel.members.size == 0) return;

            const lastChannelId = lastChannel.name.split("#")[1]
            catagory.createChannel(`Room #${Number(lastChannelId) + 1}`, { type: "GUILD_VOICE", userLimit: 99 })
        })
})

// Delete channel when all user leaves channel
client.on("voiceStateUpdate", async (oldState, newState) => {
    const guild = oldState.guild;
    const oldChannel = oldState.channel

    if (!oldChannel || oldChannel.members.size > 0) { return; }

    // Loop through channels and delete all empty channels. Then create new channel
    guild.channels.fetch("923679215205892098") // Room's Catagory
        .then(async (catagory) => {
            if (!catagory || catagory.type != "GUILD_CATEGORY") return;

            for (const child of catagory.children.values()) {
                if (child.members.size > 0) { continue; }
                await child.delete()
            }

            const lastChannel = catagory.children.sort((first, second) => { return first.name.localeCompare(second.name); }).last()
            if (!lastChannel || lastChannel.members.size == 0) return;

            const lastChannelId = lastChannel.name.split("#")[1]
            catagory.createChannel(`Room #${Number(lastChannelId) + 1}`, { type: "GUILD_VOICE", userLimit: 99 })
        })
})


client.on("disconnect", () => {
    console.log("Bot shutting down.")
})

client.on("error", error => {
    console.log(`Error Encountered ${error.message}`);
})

client.login(process.env.TOKEN)

process.on("uncaughtException", (error) => {
    fs.writeFileSync("crash.txt", `Uncaught Exception: ${error.message}`);
    console.error(error)
    process.exit(1);
})

process.on("unhandledRejection", (reason: Error, promise) => {
    fs.writeFileSync("crash.txt", `Unhandled rejection at ${promise}, reason: ${reason.message}`);
    console.error(reason)
    process.exit(1);
})