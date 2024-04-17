use crate::{
    commands::{autoreply, average, uptime},
};

use sqlx::{Row};

use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::{InteractionData},
    },
    channel::message::AllowedMentions,
    gateway::payload::incoming::InteractionCreate,
    http::interaction::{
        InteractionResponse, InteractionResponseData,
        InteractionResponseType::ChannelMessageWithSource,
    },
    id::{
        marker::{GuildMarker},
        Id,
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
            users: vec![crate::OWNER_ID],
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
        "uptime" => uptime::interaction(data, ctx).await?,
        "autoreply" => autoreply::interaction(data, ctx).await?,
        "average" => average::interaction(data, ctx).await?,
        "migrate" => migrate(ctx).await?,
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

pub async fn migrate(ctx: &Context) -> anyhow::Result<InteractionResponseData> {
    let rules = sqlx::query("select * from replies")
        .fetch_all(&ctx.db)
        .await?;

    for r in rules {
        sqlx::query("insert into rules values ($1, $2, $3)")
            .bind(r.get::<String, _>("trigger"))
            .bind(r.get::<String, _>("reply"))
            .bind(Id::<GuildMarker>::new(1023332212403351563).to_string())
            .execute(&ctx.db)
            .await?;
    }

    //pub message: String,
    //pub guild_id: Id<GuildMarker>,
    //pub message_id: Id<MessageMarker>,
    //pub post_id: Id<MessageMarker>,
    //pub stars: i32,

    for e in sqlx::query("select * from moyai")
        .fetch_all(&ctx.db)
        .await?
    {
        sqlx::query("insert into board values ($1, $2, $3, $4, $5)")
            .bind(e.get::<String, _>("message_content"))
            .bind(Id::<GuildMarker>::new(1023332212403351563).to_string())
            .bind(e.get::<String, _>("message_id"))
            .bind(e.get::<String, _>("post_id"))
            .bind(e.get::<String, _>("moyai_count"))
            .execute(&ctx.db)
            .await?;
    }

    Ok(InteractionResponseDataBuilder::new().content("ok").build())
}

pub fn commands() -> [Command; 4] {
    [
        CommandBuilder::new("uptime", "Show the bot's uptime", CommandType::ChatInput).build(),
        CommandBuilder::new("migrate", "migrate the ancient db", CommandType::ChatInput)
            .guild_id(Id::new(1089645999787610287))
            .build(),
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
