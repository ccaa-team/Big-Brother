use poise::serenity_prelude::{ChannelId, CurrentUser, PrivateChannel};
use sqlx::SqlitePool;

pub struct Data {
    pub bot_pfp: String,
    pub bot: CurrentUser,
    pub logs_channel: PrivateChannel,
    pub cursed_channel: ChannelId,
    pub db: SqlitePool,
}

pub struct BoardEntry {
    pub message_id: String,
    pub post_id: Option<String>,
    pub message_content: String,
    pub moyai_count: i64,
    pub author: String,
}

pub struct AutoReply {
    pub trigger: String,
    pub reply: String,
}
