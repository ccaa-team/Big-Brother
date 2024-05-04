use poise::serenity_prelude::{ChannelId, GuildId, MessageId, RoleId};
use sqlx::{postgres::PgRow, Row};

#[derive(Clone)]
pub struct Rule {
    pub trigger: String,
    pub reply: String,
    pub guild: GuildId,
}

impl sqlx::FromRow<'_, PgRow> for Rule {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            trigger: row.try_get("trigger")?,
            reply: row.try_get("reply")?,
            guild: row.try_get::<String, _>("guild")?.parse().unwrap(),
        })
    }
}

#[derive(Clone)]
pub struct BoardEntry {
    pub message: String,
    pub guild_id: GuildId,
    pub message_id: MessageId,
    pub post_id: Option<MessageId>,
    pub stars: i32,
}

impl sqlx::FromRow<'_, PgRow> for BoardEntry {
    fn from_row(row: &PgRow) -> Result<BoardEntry, sqlx::Error> {
        Ok(Self {
            message: row.try_get("message").unwrap(),
            guild_id: row
                .try_get::<String, _>("guild_id")?
                .as_str()
                .parse()
                .unwrap(),
            message_id: row
                .try_get::<String, _>("message_id")?
                .as_str()
                .parse()
                .unwrap(),
            post_id: row
                .try_get::<Option<String>, _>("post_id")?
                .map(|p| p.as_str().parse::<MessageId>().unwrap()),
            stars: row.try_get("stars")?,
        })
    }
}

#[derive(Clone, Default)]
pub struct Settings {
    pub guild: GuildId,
    pub board_threshold: Option<i32>,
    pub board_channel: Option<ChannelId>,
    pub reply_role: Option<RoleId>,
}

impl sqlx::FromRow<'_, PgRow> for Settings {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        // These unwraps aren't gonna fail unless the DB fails on us (or someone finds a way to
        // inject squeel)
        Ok(Self {
            guild: row.try_get::<String, _>("guild")?.as_str().parse().unwrap(),
            board_threshold: row.try_get("board_threshold").ok(),
            board_channel: row
                .try_get("board_channel")
                .ok()
                .map(|c: String| c.as_str().parse().unwrap()),
            reply_role: row
                .try_get("reply_role")
                .ok()
                .map(|c: String| c.as_str().parse().unwrap()),
        })
    }
}
