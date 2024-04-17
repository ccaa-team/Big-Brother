use sqlx::{postgres::PgRow, Row};
use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker, MessageMarker},
    Id,
};

#[derive(Clone)]
pub struct Rule {
    pub trigger: String,
    pub reply: String,
    pub guild: Id<GuildMarker>,
}

impl sqlx::FromRow<'_, PgRow> for Rule {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            trigger: row.try_get("trigger")?,
            reply: row.try_get("reply")?,
            guild: row.try_get::<String, _>("guild")?.as_str().parse().unwrap(),
        })
    }
}

#[derive(Clone)]
pub struct BoardEntry {
    pub message: String,
    pub guild_id: Id<GuildMarker>,
    pub message_id: Id<MessageMarker>,
    pub post_id: Id<MessageMarker>,
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
                .try_get::<String, _>("post_id")?
                .as_str()
                .parse()
                .unwrap(),
            stars: row.try_get("stars")?,
        })
    }
}

#[derive(Clone)]
pub struct Settings {
    pub guild: Id<GuildMarker>,
    pub board_threshold: i16,
    pub board_channel: Option<Id<ChannelMarker>>,
}

impl sqlx::FromRow<'_, PgRow> for Settings {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        // These unwraps aren't gonna fail unless the DB fails on us (or someone finds a way to
        // inject squeel)
        Ok(Self {
            guild: row.try_get::<String, _>("guild")?.as_str().parse().unwrap(),
            board_threshold: row.try_get("board_threshold")?,
            board_channel: row
                .try_get("board_channel")
                .ok()
                .map(|c: String| c.as_str().parse().unwrap()),
        })
    }
}
