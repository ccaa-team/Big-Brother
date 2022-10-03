const { SlashCommandBuilder } = require("discord.js");

async function execute(interaction) {
  let member = interaction.options.getMember("member");
  interaction.guild.roles
    .fetch("1023334181952049203")
    .then((role) => member.roles.add(role));

  interaction.reply({
    content: "The role should be added soon-ish",
    ephemeral: true,
  });
}

module.exports = {
  data: new SlashCommandBuilder()
    .setName("embrace")
    .setDescription("Gives the user member role")
    .setDefaultMemberPermissions(268435456)
    .addUserOption((option) =>
      option
        .setName("member")
        .setDescription("The member to embrace")
        .setRequired(true)
    ),
  execute,
};
