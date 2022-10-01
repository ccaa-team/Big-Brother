const { Client, GatewayIntentBits } = require("discord.js");
const Uwuifier = require("uwuifier");
const client = new Client({ intents: [GatewayIntentBits.GuildWebhooks, GatewayIntentBits.Guilds, GatewayIntentBits.GuildMessages, GatewayIntentBits.MessageContent] });
require("dotenv").config();

let uwuifier = new Uwuifier.default();

client.on("ready", () => {
  console.log(`Logged in as ${client.user.tag}`);
});

client.on("messageCreate", async msg => {
  if (msg.author.bot) return;
  if (msg.content.startsWith("uwuify")) {

    let offset = 6;

    let text = msg.cleanContent.slice(offset, msg.cleanContent.length)

    text = text.replace("@everyone", "at everyone");

    if (text.length == 0) return;

    let uwu = uwuifier.uwuifySentence(text);
    let name = uwuifier.uwuifyWords(msg.author.username);
    let avatarUrl = msg.author.avatarURL();

    let webhooks = await msg.channel.fetchWebhooks().catch(console.error);
    if (webhooks.size == 0) {
      await msg.channel.createWebhook({
        name: "UwU webhook",
        avatar: "https://media.discordapp.net/attachments/1015273149115416596/1025011813026373682/licc.png",
      }).then(console.log).catch(console.error);
      webhooks = await msg.channel.fetchWebhooks().catch(console.error); // :skull:
    }

    let webhook = webhooks.first();

    msg.delete().catch(console.err);

    webhook.send({
      content: uwu,
      username: name,
      avatarURL: avatarUrl,
    })
  }
})

client.login(process.env.token);
