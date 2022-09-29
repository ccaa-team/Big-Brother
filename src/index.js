let { Client, GatewayIntentBits } = require("discord.js");
let Uwuifier = require("uwuifier");
const client = new Client({ intents: [GatewayIntentBits.GuildWebhooks, GatewayIntentBits.Guilds, GatewayIntentBits.GuildMessages, GatewayIntentBits.MessageContent] });
require("dotenv").config();

let uwuifier = new Uwuifier.default();

let webhook_cache = {};

client.on("ready", () => {
  console.log(`Logged in as ${client.user.tag}`);
});

client.on("messageCreate", async msg => {
  if (msg.author.bot) return;
  if (msg.content.startsWith("||") || msg.author.id == "1000953755744882698") {

    let offset = 2;

    if (msg.author.id == "1000953755744882698") offset = 0;

    let text = msg.cleanContent.slice(offset, msg.cleanContent.length)

    text = text.replace("@everyone", "at everyone");

    if (text.length == 0) return;

    let uwu = uwuifier.uwuifySentence(text);
    let name = uwuifier.uwuifyWords(msg.author.username);
    let avatarUrl = msg.author.avatarURL();

    let webhooks = webhook_cache[msg.channelId]

    if (webhooks === undefined) {
      webhooks = await msg.channel.fetchWebhooks().catch(console.error);
      if (webhooks.size == 0) {
        await msg.channel.createWebhook({
          name: "UwU webhook",
          avatar: "https://media.discordapp.net/attachments/1015273149115416596/1025011813026373682/licc.png",
        }).then(console.log).catch(console.error);
        webhooks = await msg.channel.fetchWebhooks().catch(console.error); // :skull:
      }
      webhook_cache[msg.channelId] = webhooks;
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
