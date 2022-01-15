import { ReactionRole } from "./commands/reaction roles/reactionRole";

export class Server {
    id: string;
    reactionRoles: ReactionRole[]
    disabledCommands: string[];
    roles: Record<string, string[]>;
    channels: Record<string, string>;
    serverRules: Record<string, string>
    hidden: Record<string, Record<string, string>>
    moderation: Record<string, Record<string, string>>

    constructor(id: string) {
        this.id = id;
        this.reactionRoles = []
        this.disabledCommands = []
        this.roles = {}
        this.channels = {};
        this.serverRules = {}
        this.hidden = {}
        this.moderation = {}

        servers[this.id] = this
    }
}

export let servers: Record<string, Server> = {};