import Discord from "discord.js";

module.exports = function (message: Discord.Message) {
    if (!message.client.user) {
        return;
    }

    if (message.mentions.users && message.mentions.users.first()?.id == message.client.user.id && message.content.slice(-1) == "?") {
        if (Math.floor(Math.random() * 2)) {
            message.reply("Yes").then();
        } else {
            message.reply("No").then();
        }
    }
}
