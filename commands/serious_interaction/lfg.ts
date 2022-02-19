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
        {
            name: "party_size",
            description: "The size of the required party (including you)",
            type: "NUMBER"
        }
    ],
    expectedArgs: "<title>, <description>, <size>",
    permissionError: "",
    minArgs: 1,
    callback: (interaction: Discord.CommandInteraction, options: Discord.CommandInteractionOptionResolver) => {
        if (!interaction.inGuild()) { return; }
        const member = interaction.member as Discord.GuildMember

        const title = options.get("title")?.value as string
        const description = options.get("description")?.value as string
        const partySize = Number(options.get("party_size")?.value)

        const embed = new Discord.MessageEmbed()
            .setAuthor(member.displayName)
            .setTitle(title)
            .setDescription(description)
            .addField(`Accepted:${isNaN(partySize) ? "" : ` (1/${partySize})`}`, `${member}`)

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

        interaction.reply({ content: "<@&944486978693509150>", embeds: [embed], components: [row] })

        if (!interaction.channel) { return; }

        const filter = (interaction: Discord.MessageComponentInteraction) => (
            ["lfg-join", "lfg-decline"].includes(interaction.customId)
        );

        const collector = interaction.channel.createMessageComponentCollector({ filter, componentType: "BUTTON" })

        collector.on("collect", async (i) => {
            const oldEmbed = i.message.embeds[0] as Discord.MessageEmbed
            const author = oldEmbed.author?.name as string
            const title = oldEmbed.title as string
            const description = oldEmbed.description as string
            let acceptedArray = oldEmbed.fields[0] ? new Set(oldEmbed.fields[0].value.split("\n")) : new Set()
            let reservedArray = new Set()

            const new_embed = new Discord.MessageEmbed()
                .setAuthor(author)
                .setTitle(title)
                .setDescription(description)

            const stringMember = String(i.member)

            if (i.customId == join.customId) {
                if (!isNaN(partySize) && acceptedArray.size < partySize) {
                    const numberOfMembers = partySize - acceptedArray.size
                    for (let j = 0; j < numberOfMembers; j++) {
                        if (!reservedArray.size) { break; }

                        const [first] = reservedArray
                        acceptedArray.add(first)
                        reservedArray.delete(first)
                    }
                }

                if (!isNaN(partySize) && acceptedArray.size >= partySize) {
                    reservedArray.add(stringMember)
                } else {
                    acceptedArray.add(stringMember)
                }
            }

            else if (i.customId == decline.customId) {
                acceptedArray.delete(stringMember)
                reservedArray.delete(stringMember)
            }

            if (acceptedArray.size) {
                new_embed.addField(`Accepted:${isNaN(partySize) ? "" : ` (${acceptedArray.size}/${partySize})`}`, Array.from(acceptedArray).join("\n"))
            }

            if (reservedArray.size) {
                new_embed.addField("Reserved:", Array.from(reservedArray).join("\n"))
            }

            if (acceptedArray.size == partySize && !reservedArray.size) {
                const channel = i.channel
                if (channel) {
                    i.channel.send(`${i.member} you're party "${title}" is now full.`)
                }
            }

            i.update({ embeds: [new_embed], components: [row] })
        })
    }
}