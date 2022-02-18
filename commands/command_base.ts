import Discord from "discord.js"
import { servers } from "../server"
import { commands } from "./load_commands"


let recentlyRan: string[] = []

module.exports = {
    runMessageCommand: function (message: Discord.Message) {
        const { member, content, guild, channel } = message

        if (!member || !guild) { return; }

        const serverConfig = servers[guild.id]
        const botConfig = require("../configs/bot_config.json");

        const commandName = content.split(" ")[0].toLowerCase()
        if (!commandName.startsWith(botConfig.prefix)) { return; }

        const command = commands.get(commandName.slice(botConfig.prefix.length))
        if (!command || !["messageCreate", "all"].includes(command.eventListener)) { return; }

        // Check if the command is enabled in that server
        if (serverConfig.disabledCommands.includes(commandName)) { return }

        // Check if the user has the correct permissions to run the command
        for (const permission of command.permissions) {
            if (guild && !member.permissions.has(permission) && !botConfig.developers.includes(member.id)) {
                message.reply({ content: command.permissionError })
                return
            }
        }

        // Check if the user has the required roles to run the command
        for (const requiredRole of command.requiredRoles) {
            const role = guild.roles.cache.find(role => role.name === requiredRole)

            if (!role || !member.roles.cache.has(role.id) && !botConfig.developers.includes(member.id)) {
                message.reply({ content: command.permissionError })
                return
            }
        }

        // Check if the user is blacklisted
        const blacklist = require("../blacklist");
        if (guild && blacklist.isBlacklisted(member.id) && !botConfig.developers.includes(member.id)) {
            return
        }

        // Check if the command is on cooldown
        try { var cooldownString = `${guild.id}-${member.id}-${command.commands[0]}` }
        catch { var cooldownString = `privateMessage-${message.author.id}-${command.commands[0]}` }
        if (command.cooldown > 0 && recentlyRan.includes(cooldownString) && !member.roles.cache.hasAny(...serverConfig.roles.moderationRoles)) {
            message.reply("You cannot use that command so soon, please wait")
            return
        }

        // Create the arguments variable
        const args = content.split(/[ ]+/)
        args.shift()

        // Check if the user inputed the correct number of arguments
        if (args.length < command.minArgs || (command.maxArgs !== null && args.length > command.maxArgs)) {
            const embed = new Discord.MessageEmbed()
                .setColor("#ff0000")
                .setDescription(`Invalid command usage, try using it like:\n\`${botConfig.prefix}${commandName} ${command.expectedArgs}\``)

            channel.send({ embeds: [embed] });
            return
        }

        // Add command to recentlyRan if command has cooldown
        if (command.cooldown > 0) {
            recentlyRan.push(cooldownString)

            setTimeout(() => {
                recentlyRan = recentlyRan.filter((string) => {
                    return string !== cooldownString
                })
            }, 1000 * command.cooldown);
        }

        console.log(`Running ${commandName}`)
        command.callback(message, args, args.join(" "))

        return
    }
}
