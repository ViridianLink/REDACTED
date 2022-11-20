import Discord from "discord.js"
import {Redacted} from "../client";

export async function gameRoles(client: Redacted, channelId: string, messageId: string) {
    const validGames = [
        "0️⃣ - LFG Pings\n",
        "1️⃣ - CSGO",
        "2️⃣ - Dead By Daylight",
        "3️⃣ - Destiny 2",
        "4️⃣ - League of Legends",
        "5️⃣ - Overwatch",
        "6️⃣ - Rainbow Six",
        "7️⃣ - Space Engineers",
        "8️⃣ - Teamfight Tactics",
        "9️⃣ - Warframe"
    ]

    const channel = await client.channels.fetch(channelId)
    if (!(channel instanceof Discord.TextChannel)) {
        return console.error("Invalid channel id")
    }

    const embed = new Discord.EmbedBuilder()
        .setTitle("Add a game by clicking the reactions below!")
        .setDescription(validGames.join('\n'))

    channel.messages.fetch(messageId).then(message => message.edit({embeds: [embed]}))
}

export async function colourRoles(client: Redacted, channelId: string, messageId: string) {
    const channel = await client.channels.fetch(channelId)
    if (!(channel instanceof Discord.TextChannel)) {
        return console.error("Invalid channel id")
    }

    const embed = new Discord.EmbedBuilder()
        .setTitle("Add a colour by clicking the reactions below!")
        .setDescription("🔴 : `Red`\n🟠 : `Orange`\n🟡 : `Yellow`\n🟢 : `Green`\n🔵 : `Blue`\n🟣 : `Purple`\n⚪ : `White`\n⚫ : `Black`")

    channel.messages.fetch(messageId).then(message => message.edit({embeds: [embed]}))
}
