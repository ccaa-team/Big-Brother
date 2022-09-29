const { Client, GatewayIntentBits } = require("discord.js");
const { default: Uwuifier } = require("uwuifier");
const client = new Client({ intents: [GatewayIntentBits.MessageContent, GatewayIntentBits.GuildMessages, GatewayIntentBits.Guilds] });
require("dotenv").config();

let uwuifier = new Uwuifier();

client.on("ready", () => {
  console.log(`Logged in as ${client.user.tag}`);
});

client.on('interactionCreate', async interaction => {
  if (!interaction.isChatInputCommand()) return;

  let text = interaction.options.getString("text");

  let uwu = uwuifier.uwuifySentence(text);

  let webhooks = await interaction.channel.fetchWebhooks();

  let webhook;

  if (webhooks.size == 0) {
    webhook = await interaction.channel.createWebhook({
      name: "Uwu webhook",
      avatar: "https://media.discordapp.net/attachments/1015273149115416596/1025011813026373682/licc.png",
    }).catch(console.error);
  } else webhook = webhooks.first();

  webhook.send({
    content: uwu,
  })
})

client.login(process.env.token);
