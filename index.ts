import Discord from "discord.js";
import dotenv from "dotenv";
import fs from "fs";
import {loadSlashCommands} from "./commands/load_commands";
import {createServer} from "./servers";
import {Redacted} from "./client";
import deployCommands from "./deploy_commands"
import mongoose from "mongoose";
import {getServer, Server} from "./models/server";
import {getRoleResolvable} from "./common";

switch (process.env.NODE_ENV) {
    case "development":
        dotenv.config({path: "./.env.local"})
        break;
    default:
        dotenv.config()
        break;
}

const dbURI = process.env.MONGO_ATLAS_URI || ""
mongoose.connect(dbURI)
    .then(() => console.log("Connected to DB"))
    .catch(console.error)

export const client = new Redacted({
    intents: [
        Discord.GatewayIntentBits.Guilds,
        Discord.GatewayIntentBits.GuildMessages,
        Discord.GatewayIntentBits.GuildMembers,
        Discord.GatewayIntentBits.GuildMessageReactions,
        Discord.GatewayIntentBits.GuildVoiceStates,
        Discord.GatewayIntentBits.MessageContent,
    ],
    partials: [
        Discord.Partials.Message,
        Discord.Partials.Channel,
        Discord.Partials.Reaction,
    ]
})


// Init
client.on(Discord.Events.ClientReady, async () => {
    const botConfig = require("./configs/bot_config.json");
    console.log(`REDACTED is Running, version: ${botConfig.version}`);

    if (client.user) {
        client.user.setPresence({activities: [{name: "Destiny 2"}], status: "online"})
    }

    // Initialize Servers
    await require("./servers").init(client)

    loadSlashCommands(client)

    if (process.env.NODE_ENV == "development") {
        deployCommands(client).then()
    }

    // Self Updating
    const customRoles = require("./self_updating/customRoles")
    customRoles.gameRoles(client, "931986133762588752", "931988680057454592").then()
    customRoles.colourRoles(client, "932004859899691018", "932004884839039047").then()
});


client.on(Discord.Events.GuildCreate, guild => {
    createServer(guild.id)
})


client.on(Discord.Events.GuildMemberAdd, async (member) => {
    const welcomeMessage = `
**Welcome to Redacted**

Due to our growing and diverse community we've added this welcome message to give everyone the smoothest experience.

Getting Started:
1. Pick your roles in <#931986133762588752> and <#932004859899691018>
2. Read the rules in <#931986133762588752>

Some important rules:
- Please avoid any racism, sexism and other discrimination
- Due to a mental disorder called Misophonia please avoid any chewing, eating or crunching noises.
    - If you wish to eat, please mute your mic beforehand
`

    const tasks: Promise<any>[] = []
    tasks.push(member.send(welcomeMessage))

    const server = await getServer(member.guild.id)

    const defaultRoles = await getRoleResolvable(member.guild, server.roles.default)

    tasks.push(member.roles.add(defaultRoles))

    Promise.all(tasks).catch()
})

client.on(Discord.Events.MessageReactionAdd, async (reaction, user) => {
    const guild = reaction.message.guild
    if (!guild) return;

    const server = await Server.findOne({id: guild.id}).exec()
    if (!server) return;

    for (const reactionRole of server.reactionRoles) {
        if (reaction.message.id == reactionRole.messageId && reaction.emoji.toString() == reactionRole.emoji && user.id !== client.user?.id) {
            const member = guild.members.cache.find(member => member.id == user.id)
            if (!member) {
                break;
            }

            const role = await guild.roles.fetch(reactionRole.roleId)
            if (!role) {
                break;
            }

            member.roles.add(role)
                .catch((error) => console.log(error))
            break;
        }
    }
})

client.on(Discord.Events.MessageReactionRemove, async (reaction, user) => {
    const guild = reaction.message.guild
    if (!guild) return;

    const server = await Server.findOne({id: guild.id}).exec()
    if (!server) return;

    for (const reactionRole of server.reactionRoles) {
        if (reaction.message.id == reactionRole.messageId && reaction.emoji.toString() == reactionRole.emoji && user.id !== client.user?.id) {
            const member = guild.members.cache.find(member => member.id == user.id)
            if (!member) {
                break;
            }

            const role = await guild.roles.fetch(reactionRole.roleId)
            if (!role) {
                break;
            }

            member.roles.remove(role)
                .catch((error) => console.log(error))
            break;
        }
    }
})

client.on(Discord.Events.InteractionCreate, async interaction => {
    if (!interaction.isChatInputCommand()) return;

    const client = interaction.client as Redacted
    const command = client.slashCommands?.get(interaction.commandName);

    if (!command) {
        console.error(`No command matching ${interaction.commandName} was found.`);
        return;
    }

    try {
        await command.execute(interaction)
    } catch (error) {
        console.error(error);
        await interaction.reply({
            content: "There was an error while executing this command!",
            ephemeral: true
        })
    }

})

client.on(Discord.Events.VoiceStateUpdate, async (oldState, newState) => {
    const roomsCategory = await oldState.guild.channels.fetch("923679215205892098") || await newState.guild.channels.fetch("923679215205892098")

    if (roomsCategory?.type != Discord.ChannelType.GuildCategory) {
        return console.error("Invalid rooms Category")
    }

    if (oldState.channel && oldState.channel.members.size == 0) {
        const emptyRooms = roomsCategory.children.cache.filter(child => child.members.size == 0 && child.name != "Room #1")

        await Promise.all(emptyRooms.map(child => child.delete()))
    }

    const lastChannel = roomsCategory.children.cache.last()

    if (!lastChannel || lastChannel.members.size == 0) {
        return;
    }

    const lastChannelId = lastChannel.name.split("#")[1]

    roomsCategory.children.create({
        name: `Room #${Number(lastChannelId) + 1}`,
        type: Discord.ChannelType.GuildVoice,
        userLimit: 99
    }).then()
})

client.on(Discord.Events.Error, error => {
    console.log(`Error Encountered ${error.message}`);
})

client.login(process.env.TOKEN).then()

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
