const { SlashCommandBuilder } = require("discord.js");
const Uwuifier = require("uwuifier");

let uwuifier = new Uwuifier.default({
  spaces: {
    faces: 0.125,
    actions: 0,
    stutters: 0.125,
  },
  words: 1,
  exclamations: 1,
});

async function execute(interaction) {
  let text = interaction.options.getString("text");

  text = text.replace("@everyone", "at everyone");

  let uwu = uwuifier.uwuifySentence(text);

  if (uwu.length > 2000) {
    interaction.reply("Your message was too long.");
  }

  let name = uwuifier.uwuifyWords(interaction.user.username);
  let avatarUrl = interaction.user.avatarURL();

  let webhooks = await interaction.channel.fetchWebhooks().catch(console.error);
  if (webhooks.size == 0) {
    await interaction.channel
      .createWebhook({
        name: "UwU webhook",
        avatar:
          "https://media.discordapp.net/attachments/1015273149115416596/1025011813026373682/licc.png",
      })
      .then(console.log)
      .catch(console.error);
    webhooks = await interaction.channel.fetchWebhooks().catch(console.error); // :skull:
  }

  let webhook = webhooks.first();

  webhook.send({
    content: uwu,
    username: name,
    avatarURL: avatarUrl,
  });

  await interaction.reply({ content: "ok.", ephemeral: true });
}

module.exports = {
  data: new SlashCommandBuilder()
    .setName("uwu")
    .setDescription("UwUifies your text.")
    .addStringOption(option =>
      option
        .setName("text")
        .setDescription("The text to uwuify")
        .setRequired(true)
    ),
  execute,
};
