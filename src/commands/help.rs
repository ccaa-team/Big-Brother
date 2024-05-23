use crate::{Context, Error};
use poise::command;

#[command(slash_command, prefix_command, hide_in_help, track_edits)]
/// The help command
///
/// Uses `poise::samples::help` under the hood
pub async fn help(ctx: Context<'_>, #[rest] command: Option<String>) -> Result<(), Error> {
    let config = poise::samples::HelpConfiguration {
        extra_text_at_bottom:
            "Use `;help command` for a specific command, you can also edit your message.",
        ephemeral: true,
        //show_context_menu_commands: true,
        show_subcommands: true,
        include_description: true,
        ..Default::default()
    };

    poise::builtins::help(ctx, command.as_deref(), config).await?;

    Ok(())
}
