use nekosbest::Category;
use poise::command;

use crate::{Context, Error};

lazy_static::lazy_static! {
static ref CATEGORIES: Vec<&'static str> = Category::ALL_VARIANTS
    .iter()
    .map(|v| v.to_url_name())
    .collect();
}

async fn autocomplete_category<'a>(_ctx: Context<'_>, partial: &'a str) -> Vec<String> {
    CATEGORIES
        .iter()
        .filter(|v| partial.is_empty() || v.starts_with(partial))
        .map(|v| v.to_string())
        .collect()
}

#[command(slash_command, prefix_command)]
pub async fn neko(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_category"] category: String,
) -> Result<(), Error> {
    let category = if CATEGORIES.contains(&category.as_str()) {
        Category::from_url_name(&category).unwrap()
    } else {
        ctx.send(|m| {
            m.content(format!(
                "{} is not a valid category, use the slash command for a list",
                category
            ))
        })
        .await?;
        return Ok(());
    };

    let url = nekosbest::get(category).await?.url;

    ctx.send(|m| m.embed(|e| e.image(url))).await?;

    Ok(())
}
