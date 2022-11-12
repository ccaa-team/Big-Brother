const { Client, GatewayIntentBits, Collection, Events } = require("discord.js");
const fs = require("node:fs");
const path = require("node:path");

const licc_emoji = "<:whatwedotoyourballs:1023352899075571752>"

const client = new Client({
  intents: [GatewayIntentBits.GuildWebhooks, GatewayIntentBits.Guilds, GatewayIntentBits.MessageContent, GatewayIntentBits.GuildMessages],
});

client.commands = new Collection();
const commandsPath = path.join(__dirname, "commands");
console.log(commandsPath);
const commandFiles = fs
  .readdirSync(commandsPath)
  .filter((file) => file.endsWith(".js"));

for (const file of commandFiles) {
  const filePath = path.join(commandsPath, file);
  const command = require(filePath);

  client.commands.set(command.data.name, command);
}

require("dotenv").config();

client.on("ready", () => {
  console.log(`Logged in as ${client.user.tag}`);
});

client.on("interactionCreate", async (interaction) => {
  if (!interaction.isChatInputCommand()) return;

  const command = interaction.client.commands.get(interaction.commandName);

  if (!command) return;

  try {
    await command.execute(interaction);
  } catch (error) {
    console.error(error);
    await interaction.reply({ content: `Error: ${error}`, ephemeral: true });
  }
});

client.on(Events.MessageCreate, msg => {
  if (msg.content.match(/ba[lw][lw]s/gi)) {
    msg.react(licc_emoji);
  }

  if (msg.content.match("🗿")) {
    msg.react("🗿");
  }

  let roll = Math.random();

  if (roll >= 0.99) {
    msg.reply("*Pees in your ass*");
  }
})

client.login(process.env.token);
