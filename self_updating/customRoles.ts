import Discord from "discord.js"

module.exports = {
    gameRoles: async function (client: Discord.Client, channelId: string, messageId: string) {
        const channel = await client.channels.fetch(channelId)
        if (!channel || !channel.isText()) { return console.error("Invalid channel id") }

        const embed = new Discord.MessageEmbed()
            .setTitle("Join a game by clicking the icons below!")
            .setDescription("1️⃣ - Destiny 2\n2️⃣ - League of League\n3️⃣ - Rainbow Six Siege\n4️⃣ - Teamfight Tactics\n5️⃣ - Phasmophoba\n6️⃣ - PUBG")

        channel.messages.fetch(messageId).then((message) => { message.edit({ embeds: [embed] }) })
    }
} 
