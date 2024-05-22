use poise::{
    command, send_reply,
    serenity_prelude::{CreateAllowedMentions, CreateEmbed},
    CreateReply,
};
use sqlx::query;

use crate::{
    structs::Rule,
    utils::{get_settings, truncate, EMBED_AUTHOR, EMBED_COLOR},
    Context, Error,
};

#[command(
    slash_command,
    prefix_command,
    guild_only,
    subcommand_required,
    subcommands("add", "remove", "list"),
    ephemeral,
    track_edits
)]
// The main autoreply command, mainly shown to show the rest
pub async fn autoreply(_: Context<'_>) -> Result<(), Error> {
    unreachable!()
}

async fn role(ctx: Context<'_>) -> Result<bool, Error> {
    let settings = get_settings(ctx.guild_id().unwrap(), &ctx.data().db).await?;

    match settings.reply_role {
        None => Ok(true),
        Some(r) => {
            let user = ctx.author_member().await.unwrap();
            if !user.roles.contains(&r) {
                let msg = CreateReply::default()
                    .content(format!(
                        "You're missing the role required for the command, are you sure you have <@&{r}>?"
                    ))
                    .allowed_mentions(CreateAllowedMentions::new().empty_roles());
                send_reply(ctx, msg).await?;
                return Ok(false);
            }
            Ok(true)
        }
    }
}

#[command(slash_command, prefix_command, guild_only, ephemeral, check = "role")]
/// Add an autoreply
///
/// `;autoreply add "me when" "me when the"`
async fn add(
    ctx: Context<'_>,
    #[description = "The trigger text"] trigger: String,
    #[description = "Text to reply with"]
    #[rest]
    reply: String,
) -> Result<(), Error> {
    let trigger = trigger.to_lowercase();
    let guild = ctx.guild_id().unwrap();
    if ctx
        .data()
        .rules
        .read()
        .unwrap()
        .iter()
        .any(|r| r.trigger == trigger && r.guild == guild)
    {
        ctx.reply("Rule already exists, delete it if you want to replace it.")
            .await?;
        return Ok(());
    };

    query("insert into rules values ($1, $2, $3)")
        .bind(&trigger)
        .bind(&reply)
        .bind(ctx.guild_id().unwrap().to_string())
        .execute(&ctx.data().db)
        .await?;

    let out = format!("Added rule `{}`!", trigger);

    ctx.data().rules.write().unwrap().push(Rule {
        trigger,
        reply,
        guild,
    });

    ctx.reply(out).await?;

    Ok(())
}

#[command(slash_command, prefix_command, guild_only, ephemeral, check = "role")]
/// Remove an autoreply
///
/// `;autoreply remove balls`
async fn remove(
    ctx: Context<'_>,
    #[description = "The trigger text"] trigger: String,
) -> Result<(), Error> {
    let trigger = trigger.to_lowercase();
    let guild = ctx.guild_id().unwrap();
    if !ctx
        .data()
        .rules
        .read()
        .unwrap()
        .iter()
        .any(|r| r.guild == guild && r.trigger == trigger)
    {
        ctx.reply("The rule you're trying to remove doesn't exist.")
            .await?;
        return Ok(());
    };

    query("delete from rules where trigger = $1 and guild = $2")
        .bind(&trigger)
        .bind(guild.to_string())
        .execute(&ctx.data().db)
        .await?
        .rows_affected();

    {
        let mut rules = ctx.data().rules.write().unwrap();
        *rules = (*rules)
            .iter()
            .filter_map(|r| {
                if r.trigger != trigger || r.guild != guild {
                    Some(r.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
    }

    poise::say_reply(ctx, format!("Removed rule `{trigger}`")).await?;
    Ok(())
}
#[command(slash_command, prefix_command, guild_only, ephemeral)]
/// List all autoreplies for the current guild.
async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let mut out = String::new();
    let guild = ctx.guild_id().unwrap();

    {
        let rules = ctx.data().rules.read().unwrap();

        rules.iter().filter(|r| r.guild == guild).for_each(|r| {
            out += &format!(
                "\n- {}\n- - {}",
                r.trigger,
                truncate(&r.reply, 64).replace('\n', "- - ")
            )
        });
    }

    let embed = CreateEmbed::new()
        .author(EMBED_AUTHOR.to_owned())
        .color(EMBED_COLOR)
        .description(out);
    poise::send_reply(
        ctx,
        CreateReply::default()
            .reply(true)
            .ephemeral(true)
            .allowed_mentions(CreateAllowedMentions::new().replied_user(false))
            .embed(embed),
    )
    .await?;

    Ok(())
}
