import { REST } from "@discordjs/rest";
import { Routes } from 'discord-api-types/v9';
import Discord from "discord.js";
import { ICommand } from "./command";
import { commands } from "./load_commands";

const typeMap = new Map([
    ["SUB_COMMAND", 1],
    ["SUB_COMMAND_GROUP", 2],
    ["STRING", 3],
    ["INTEGER", 4],
    ["BOOLEAN", 5],
    ["USER", 6],
    ["CHANNEL", 7],
    ["ROLE", 8],
    ["MENTIONABLE", 9],
    ["NUMBER", 10],
    ["ATTACHMENT", 11]
])

module.exports = {
    deployCommands: async function (client: Discord.Client) {
        if (!client.token || !client.user) { return; }

        const rest = new REST({ version: "9" }).setToken(client.token);

        const commandArray = Array.from(commands.filter((command: ICommand) => command.description != "").values());
        const applicationCommands: any[] = []

        for (const command of commandArray) {
            const commandBody = {
                name: command.commands[0],
                description: command.description,
                options: command.options ? command.options : []
            }

            if (applicationCommands.some(command => command.name == commandBody.name)) { continue; }

            for (const option of commandBody.options) {
                const optionType = String(option.type)

                if (!typeMap.has(optionType)) { continue; }

                const typeInt = typeMap.get(optionType)

                if (typeInt === undefined) {
                    throw Error("Unknown command option '" + option.name + "'")
                }

                option.type = typeInt

            }

            applicationCommands.push(commandBody)
        }

        await rest.put(Routes.applicationCommands(client.user.id), { body: applicationCommands })
            .then(() => console.log(`Successfully registered ${applicationCommands.length} application commands.`))
            .catch(console.error);
    }
}
