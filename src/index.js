let { Client, GatewayIntentBits } = require("discord.js");
let Uwuifier = require("uwuifier");
const client = new Client({ intents: [GatewayIntentBits.GuildWebhooks, GatewayIntentBits.Guilds, GatewayIntentBits.GuildMessages, GatewayIntentBits.MessageContent] });
require("dotenv").config();

let uwuifier = new Uwuifier.default();

client.on("ready", () => {
  console.log(`Logged in as ${client.user.tag}`);
});

client.on("messageCreate", async msg => {
  if (msg.author.bot) return;
  if (msg.content.startsWith("[") && msg.content.endsWith("]")) {

    let text = msg.cleanContent.slice(1, msg.cleanContent.length - 1)

    if (text.length == 0) return;

    let uwu = uwuifier.uwuifySentence(text);
    let name = uwuifier.uwuifyWords(msg.author.username);
    let avatarUrl = msg.author.avatarURL();

    let webhooks = await msg.channel.fetchWebhooks().catch(console.error);
    let webhook = webhooks.first();

    await msg.delete();

    webhook.send({
      content: uwu,
      username: name,
      avatarURL: avatarUrl,
    })
  }
})

client.login(process.env.token);
