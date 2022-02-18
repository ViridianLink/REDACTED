import Discord from "discord.js";

interface ICommandOptions {
    choices: string[] | undefined;
    autocomplete: string | undefined;
    type: string | number;
    name: string;
    description: string;
    required: boolean;
}

export interface ICommand {
    eventListener: string;
    commands: string | string[]
    description: string
    options: ICommandOptions[] | undefined
    permissionError: string
    expectedArgs: string
    minArgs: number
    maxArgs: number | null
    cooldown: number
    permissions: Array<Discord.PermissionResolvable>
    requiredRoles: string | string[]
    callback: Function
}