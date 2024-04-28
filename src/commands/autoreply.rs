use std::collections::VecDeque;

use crate::{context::Context, structs::Rule};
use sqlx::query;
use twilight_model::{
    application::interaction::application_command::{
        CommandData, CommandDataOption, CommandOptionValue,
    },
    channel::message::MessageFlags,
    gateway::payload::incoming::InteractionCreate,
    http::interaction::InteractionResponseData,
    id::{marker::GuildMarker, Id},
};
use twilight_util::builder::InteractionResponseDataBuilder;

async fn add(
    trigger: String,
    reply: String,
    guild: Id<GuildMarker>,
    ctx: &Context,
) -> anyhow::Result<String> {
    let trigger = trigger.to_lowercase();
    if ctx
        .data
        .read()
        .await
        .rules
        .iter()
        .any(|r| r.trigger == trigger && r.guild == guild)
    {
        return Ok("Rule already exists, delete it if you want to replace it.".to_owned());
    };

    query("insert into rules values ($1, $2, $3)")
        .bind(&trigger)
        .bind(&reply)
        .bind(guild.to_string())
        .execute(&ctx.db)
        .await?;

    let out = format!("Added rule `{}`!", trigger);

    ctx.data.write().await.rules.push(Rule {
        trigger,
        reply,
        guild,
    });

    Ok(out)
}
async fn remove(trigger: String, guild: Id<GuildMarker>, ctx: &Context) -> anyhow::Result<String> {
    let trigger = trigger.to_lowercase();
    if !ctx
        .data
        .read()
        .await
        .rules
        .iter()
        .any(|r| r.guild == guild && r.trigger == trigger)
    {
        return Ok("The rule you're trying to remove doesn't exist.".to_owned());
    };

    query("delete from rules where trigger = $1 and guild = $2")
        .bind(&trigger)
        .bind(guild.to_string())
        .execute(&ctx.db)
        .await?
        .rows_affected();

    let mut data = ctx.data.write().await;
    data.rules = data
        .rules
        .iter()
        .filter_map(|r| {
            if r.trigger != trigger || r.guild != guild {
                Some(r.clone())
            } else {
                None
            }
        })
        .collect();

    Ok(format!("Removed rule `{trigger}`"))
}
fn truncate(s: &String) -> String {
    let mut s = s.to_owned();
    if s.chars().count() > 64 {
        // if this fails i'm killing myself
        let char = s.char_indices().nth(63).unwrap();
        s.truncate(char.0);
    }
    s
}
async fn list(guild: Id<GuildMarker>, ctx: &Context) -> anyhow::Result<String> {
    let mut out = String::new();

    let data = ctx.data.read().await;
    let rules = data.rules.iter().filter(|r| r.guild == guild);

    rules.for_each(|r| {
        out += &format!(
            "\n- {}\n- - {}",
            r.trigger,
            truncate(&r.reply).replace('\n', "- - ")
        )
    });

    Ok(out)
}
pub async fn interaction(
    cmd: &CommandData,
    int: &InteractionCreate,
    ctx: &Context,
) -> anyhow::Result<InteractionResponseData> {
    let get_str = |o: CommandDataOption| -> String {
        if let CommandOptionValue::String(s) = o.value {
            s
        } else {
            unreachable!()
        }
    };
    // This is a subcommand, it'll always be here
    let subcommand = &cmd.options[0];
    let mut args: VecDeque<_> = match &subcommand.value {
        CommandOptionValue::SubCommand(c) => c.clone(),
        _ => unreachable!(),
    }
    .into();
    let (trigger, reply) = (args.pop_front().map(get_str), args.pop_front().map(get_str));

    Ok(InteractionResponseDataBuilder::new()
        .content(match subcommand.name.as_str() {
            "add" => {
                add(
                    trigger.unwrap(),
                    reply.unwrap(),
                    int.guild_id.expect("this is only going to run in a guild"),
                    ctx,
                )
                .await
            }
            "remove" => remove(trigger.unwrap(), int.guild_id.expect("see above"), ctx).await,
            "list" => list(int.guild_id.expect("see abover"), ctx).await,
            _ => unreachable!(),
        }?)
        .flags(MessageFlags::EPHEMERAL)
        .build())
}
