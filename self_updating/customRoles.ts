import Discord from "discord.js"

module.exports = {
    gameRoles: async function (client: Discord.Client, channelId: string, messageId: string) {
        const channel = await client.channels.fetch(channelId)
        if (!channel || !channel.isText()) { return console.error("Invalid channel id") }

        const embed = new Discord.MessageEmbed()
            .setTitle("Add a game by clicking the reactions below!")
            .setDescription("0️⃣ - LFG Pings\n\n1️⃣ - Destiny 2\n2️⃣ - League of League\n3️⃣ - Rainbow Six Siege\n4️⃣ - Teamfight Tactics\n5️⃣ - Phasmophoba\n6️⃣ - PUBG")

        channel.messages.fetch(messageId).then((message) => { message.edit({ embeds: [embed] }) })
    },

    colourRoles: async function (client: Discord.Client, channelId: string, messageId: string) {
        const channel = await client.channels.fetch(channelId)
        if (!channel || !channel.isText()) { return console.error("Invalid channel id") }

        const embed = new Discord.MessageEmbed()
            .setTitle("Add a colour by clicking the reactions below!")
            .setDescription("🔴 : `Red`\n🟠 : `Orange`\n🟡 : `Yellow`\n🟢 : `Green`\n🔵 : `Blue`\n🟣 : `Purple`\n⚪ : `White`\n⚫ : `Black`")

        channel.messages.fetch(messageId).then((message) => { message.edit({ embeds: [embed] }) })
    }
} 
