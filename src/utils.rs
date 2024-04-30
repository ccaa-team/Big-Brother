use lazy_static::lazy_static;
use twilight_model::{
    channel::message::{embed::EmbedAuthor, Embed},
    guild::PartialMember,
    id::{
        marker::{GuildMarker, RoleMarker, UserMarker},
        Id,
    },
};
pub(crate) const OWNER_ID: Id<UserMarker> = Id::new(852877128844050432);
pub(crate) const TEST_GUILD: Id<GuildMarker> = Id::new(1089645999787610287);
pub(crate) const EMBED_COLOR: u32 = 0x7e68d0;
lazy_static! {
    pub(crate) static ref EMBED_AUTHOR: EmbedAuthor = EmbedAuthor {
        icon_url: Some("https://thevirt.ru/bot_pfp.png".to_owned()),
        name: "AutoVirt".to_owned(),
        proxy_icon_url: None,
        url: None,
    };
}

pub(crate) fn role_check(member: PartialMember, role: Id<RoleMarker>) -> Result<(), String> {
    if let Some(user) = member.user {
        if user.id == OWNER_ID {
            return Ok(());
        }
    }

    if member.roles.iter().any(|r| *r == role) {
        return Ok(());
    }

    Err("You do not have the role required to use this, sowwy! >.<".to_string())
}

#[inline(always)]
pub(crate) fn error_embed(err: String) -> Embed {
    Embed {
        author: Some(EMBED_AUTHOR.to_owned()),
        color: Some(EMBED_COLOR),
        description: Some(err),
        fields: vec![],
        footer: None,
        image: None,
        kind: "rich".to_owned(),
        provider: None,
        thumbnail: None,
        timestamp: None,
        title: None,
        url: None,
        video: None,
    }
}
