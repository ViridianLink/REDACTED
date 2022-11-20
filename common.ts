import Discord from "discord.js";

export function parseId(id: string): string | undefined {
    const match = id.match(/\d+/)
    if (match) {
        return match[0];
    }
}

export async function getRoleResolvable(guild: Discord.Guild, roleIds: string[]): Promise<Discord.RoleResolvable[]> {
    let roles: Discord.RoleResolvable[] = []

    for (let roleId of roleIds) {
        const role = await guild.roles.fetch(roleId)
        if (!role) continue;

        roles.push(role)
    }
    return roles
}
