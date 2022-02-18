import Discord from "discord.js";
import fs from "fs";
import path from "path";
import { ICommand } from "./command";

function commandBase(commandOptions: ICommand): ICommand {
    let {
        eventListener = "all",
        commands,
        description = "",
        options,
        expectedArgs = "",
        permissionError = "You do not have permission to run this command",
        minArgs = 0,
        maxArgs = null,
        cooldown = -1,
        permissions = [],
        requiredRoles = [],
        callback
    } = commandOptions

    if (typeof (commands) == "string") {
        commands = [commands]
    }

    if (permissionError.length == 0) {
        permissionError = "You do not have permission to run this command"
    }

    if (typeof (requiredRoles) == "string") {
        requiredRoles = [requiredRoles]
    }

    return { eventListener, commands, description, options, expectedArgs, permissionError, minArgs, maxArgs, cooldown, permissions, requiredRoles, callback }
}

function loadCommands(): Discord.Collection<string, ICommand> {
    const ignoreFiles = ["command_base.ts", "load_commands.ts", "command.ts"]
    const commands = new Discord.Collection<string, ICommand>()

    function readCommands(dir: string) {
        const commandFiles = fs.readdirSync(path.join(__dirname, dir))

        for (const file of commandFiles) {
            if (file.startsWith("__") || ignoreFiles.includes(file)) { continue; }

            const stat = fs.lstatSync(path.join(__dirname, dir, file))
            if (stat.isDirectory()) {
                readCommands(path.join(dir, file))
                continue
            }

            let commandOptions: ICommand = require(path.join(__dirname, dir, file))
            commandOptions = commandBase(commandOptions)

            if (commandOptions.commands && commandOptions.callback) {
                for (const c of commandOptions.commands) {
                    commands.set(c, commandOptions)
                }
            }
        }
    }

    readCommands("./")
    return commands
}

export const commands = loadCommands()
