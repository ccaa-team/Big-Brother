const { REST, SlashCommandBuilder, Routes } = require("discord.js");
require("dotenv").config();

const commands = [
  new SlashCommandBuilder()
    .setName("uwu")
    .setDescription("UwUifies your text.")
    .addStringOption((option) =>
      option
        .setName("text")
        .setDescription("The text to uwuify")
        .setRequired(true)
    ),
].map((command) => command.toJSON());

const rest = new REST({ version: "10" }).setToken(process.env.token);

rest
  .put(
    Routes.applicationGuildCommands(process.env.clientId, process.env.guildId),
    {
      body: commands,
    }
  )
  .then((data) =>
    console.log(`Successfully registered ${data.length} application commands.`)
  )
  .catch(console.error);
