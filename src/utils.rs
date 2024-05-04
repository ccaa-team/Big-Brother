use lazy_static::lazy_static;
use poise::serenity_prelude::{CreateEmbedAuthor, UserId};

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
