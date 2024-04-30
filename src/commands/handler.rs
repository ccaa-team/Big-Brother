use crate::{
    commands::{autoreply, average, uptime},
    utils::OWNER_ID,
};

use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::InteractionData,
    },
    channel::message::AllowedMentions,
    gateway::payload::incoming::InteractionCreate,
    http::interaction::{
        InteractionResponse, InteractionResponseData,
        InteractionResponseType::ChannelMessageWithSource,
    },
};
use twilight_util::builder::{
    command::{CommandBuilder, StringBuilder, SubCommandBuilder},
    InteractionResponseDataBuilder,
};

use crate::context::Context;

async fn todo(name: &str) -> InteractionResponseData {
    InteractionResponseDataBuilder::new()
        .content(format!(
            "<@!852877128844050432>, you idiot, implement {name}"
        ))
        .allowed_mentions(AllowedMentions {
            parse: vec![],
            replied_user: false,
            roles: vec![],
            users: vec![OWNER_ID],
        })
        .build()
}

pub async fn interaction(int: Box<InteractionCreate>, ctx: &Context) -> anyhow::Result<()> {
    let data = if let Some(InteractionData::ApplicationCommand(data)) = &int.data {
        data
    } else {
        unreachable!();
    };

    //let response_data = crate::autoreply::interaction(data, ctx).await?;
    let response_data = match data.name.as_str() {
        "uptime" => uptime::interaction(data, &int, ctx).await?,
        "autoreply" => autoreply::interaction(data, &int, ctx).await?,
        "average" => average::interaction(data, &int, ctx).await?,
        x => todo(x).await,
    };

    let response = InteractionResponse {
        kind: ChannelMessageWithSource,
        data: Some(response_data),
    };

    ctx.interaction()
        .create_response(int.id, &int.token, &response)
        .await?;

    Ok(())
}

pub fn commands() -> [Command; 3] {
    [
        CommandBuilder::new("uptime", "Show the bot's uptime", CommandType::ChatInput).build(),
        CommandBuilder::new(
            "autoreply",
            "How are you reading this?",
            CommandType::ChatInput,
        )
        .dm_permission(false)
        .option(
            SubCommandBuilder::new("add", "Add an autoreply rule.")
                .option(
                    StringBuilder::new("trigger", "String that triggers a reply")
                        .required(true)
                        .max_length(512),
                )
                .option(
                    StringBuilder::new("reply", "String to reply with.")
                        .required(true)
                        .max_length(256),
                ),
        )
        .option(
            SubCommandBuilder::new("remove", "Remove an autoreply rule.").option(
                StringBuilder::new("trigger", "Trigger of the reply.")
                    .required(true)
                    .max_length(512),
            ),
        )
        .option(SubCommandBuilder::new("list", "List all autoreply rules."))
        .build(),
        CommandBuilder::new(
            "average",
            "Calculate the average of something.",
            CommandType::ChatInput,
        )
        .option(StringBuilder::new("array", "Number array, separated by `;`.").required(true))
        .build(),
    ]
}
