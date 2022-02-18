import Discord from "discord.js";


// Template Command
module.exports = {
    eventListener: "interactionCreate",
    commands: ["lfg"],
    description: "Creates new LFG post",
    options: [
        {
            name: "title",
            description: "Enter LFG title",
            required: true,
            type: "STRING"
        },
        {
            name: "description",
            description: "Enter LFG description",
            required: true,
            type: "STRING"
        },
    ],
    expectedArgs: "<title>, <description>",
    permissionError: "",
    minArgs: 1,
    callback: (interaction: Discord.CommandInteraction, options: Discord.CommandInteractionOptionResolver) => {
        if (!interaction.inGuild()) { return; }
        const member = interaction.member as Discord.GuildMember

        const title = options.get("title")?.value as string
        const description = options.get("description")?.value as string

        const embed = new Discord.MessageEmbed()
            .setAuthor(member.displayName)
            .setTitle(title)
            .setDescription(description)
            .addField("Accepted:", `${member}`)

        const join = new Discord.MessageButton()
            .setCustomId("lfg-join")
            .setLabel("Join LFG")
            .setStyle("SUCCESS")

        const decline = new Discord.MessageButton()
            .setCustomId("lfg-decline")
            .setLabel("Decline")
            .setStyle("DANGER")

        const row = new Discord.MessageActionRow()
            .addComponents(join, decline)

        interaction.reply({ embeds: [embed], components: [row] })

        if (!interaction.channel) { return; }

        const filter = (interaction: Discord.MessageComponentInteraction) => (
            ["lfg-join", "lfg-decline"].includes(interaction.customId)
        );

        const collector = interaction.channel.createMessageComponentCollector({ filter, componentType: "BUTTON" })

        collector.on("collect", async (i) => {
            const old_embed = i.message.embeds[0] as Discord.MessageEmbed
            const author = old_embed.author?.name as string
            const title = old_embed.title as string
            const description = old_embed.description as string
            let accepted_array = old_embed.fields[0] ? new Set(old_embed.fields[0].value.split("\n")) : new Set()

            const new_embed = new Discord.MessageEmbed()
                .setAuthor(author)
                .setTitle(title)
                .setDescription(description)

            const string_member = String(i.member)

            if (i.customId == join.customId) {
                accepted_array.add(string_member)

                new_embed.addField("Accepted:", Array.from(accepted_array).join("\n"))

                i.update({ embeds: [new_embed], components: [row] })
            }

            else if (i.customId == decline.customId) {
                accepted_array.delete(string_member)

                if (accepted_array.size) {
                    new_embed.addField("Accepted:", Array.from(accepted_array).join("\n"))
                }

                i.update({ embeds: [new_embed], components: [row] })
            }
        })
    }
}