use poise::{
    command, say_reply, send_reply,
    serenity_prelude::{CreateEmbed, CreateEmbedFooter},
    CreateReply,
};

use crate::{
    mommy,
    utils::{EMBED_AUTHOR, EMBED_COLOR},
    Context, Error,
};

fn capitalize(str: impl Into<String>) -> String {
    let s: String = str.into();
    if s.is_empty() {
        return s;
    }
    // we only get ascii anyway
    let (left, right) = s.split_at(1);

    left.to_uppercase() + right
}

#[command(prefix_command, slash_command, aliases("define"))]
/// Fetch a definition from the urban dictionary
///
/// You can also use define instead of urban to call this.
///
/// `;urban imouto`
pub async fn urban(
    ctx: Context<'_>,
    #[rest]
    #[description = "The word(s) to define."]
    word: String,
) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let definitions: Vec<_> = urban_rs::fetch_definition(&client, &word).await?;

    if definitions.is_empty() {
        say_reply(ctx, "Couldn't find any definitions, maybe check spelling?").await?;
    } else {
        let d = &definitions[0];
        let embed = CreateEmbed::new()
            .author(EMBED_AUTHOR.to_owned())
            .title(capitalize(d.word()))
            .description(d.definition())
            .field("Example", d.example(), false)
            .footer(
                CreateEmbedFooter::new("Provided via the Urban Dictionary")
                    .icon_url("https://www.urbandictionary.com/favicon-32x32.png"),
            )
            .color(EMBED_COLOR.to_owned());

        send_reply(
            ctx,
            CreateReply::default().embed(embed).content(mommy::praise()),
        )
        .await?;
    }
    Ok(())
}
