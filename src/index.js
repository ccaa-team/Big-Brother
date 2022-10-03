const { Client, GatewayIntentBits } = require("discord.js");
const Uwuifier = require("uwuifier");
const client = new Client({
  intents: [GatewayIntentBits.GuildWebhooks, GatewayIntentBits.Guilds],
});
require("dotenv").config();

let uwuifier = new Uwuifier.default();

client.on("ready", () => {
  console.log(`Logged in as ${client.user.tag}`);
});

client.on("interactionCreate", async interaction => {
  let text = interaction.options.getString("text");

  text = text.replace("@everyone", "at everyone");

  if (text.length == 0) return;

  let uwu = uwuifier.uwuifySentence(text);
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
});

client.login(process.env.token);
