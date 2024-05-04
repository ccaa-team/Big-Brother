use lazy_static::lazy_static;
use poise::serenity_prelude::{CreateEmbedAuthor, GuildId, UserId};
use sqlx::{query_as, PgPool};

use crate::structs::Settings;

pub(crate) const OWNER_ID: UserId = UserId::new(852877128844050432);
//pub(crate) const TEST_GUILD: GuildId = GuildId::new(1089645999787610287);
pub(crate) const EMBED_COLOR: u32 = 0x7e68d0;
pub(crate) const PREFIX: &str = {
    if cfg!(debug_assertions) {
        "'"
    } else {
        ";"
    }
};
pub(crate) const PFP: &str = "https://thevirt.ru/bot_pfp.png";
lazy_static! {
    pub(crate) static ref EMBED_AUTHOR: CreateEmbedAuthor =
        CreateEmbedAuthor::new("AutoVirt").icon_url(PFP);
}

//#[inline(always)]
//pub(crate) fn error_embed(err: impl Into<String>) -> CreateEmbed {
//    CreateEmbed::new()
//        .author(EMBED_AUTHOR.clone())
//        .color(EMBED_COLOR)
//        .description(err.into())
//}

pub async fn get_settings(guild: GuildId, db: &PgPool) -> anyhow::Result<Settings> {
    let settings = query_as("select * from settings where guild = $1")
        .bind(guild.to_string())
        .fetch_optional(db)
        .await?;

    if settings.is_none() {
        sqlx::query("insert into settings(guild) values($1)")
            .bind(guild.to_string())
            .execute(db)
            .await?;
    }

    Ok(settings.unwrap_or_else(|| Settings {
        guild,
        ..Default::default()
    }))
}
