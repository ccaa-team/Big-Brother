const { REST, Routes, SlashCommandBuilder } = require("discord.js");

const commands = [
  new SlashCommandBuilder().setName("uwu").setDescription("Uwuifies your text").addStringOption(option =>
    option.setName("text")
      .setDescription("The text to uwuify")
      .setRequired(true))
].map(command => command.toJSON());

const rest = new REST({ version: "10" }).setToken(process.env.token);

rest.put(Routes.applicationGuildCommands("1023552661473210468", "1023332212403351563"), { body: commands })
  .then((data) => console.log(`Successfully registered ${data.length} application commands.`))
  .catch(console.error);
