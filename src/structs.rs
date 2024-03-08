use sqlx::PgPool;
use tokio::sync::Mutex;

use chrono::{DateTime, Local};

use poise::serenity_prelude::{ChannelId, CurrentUser, PrivateChannel};

pub struct Data {
    pub bot_pfp: Option<String>,
    pub bot: CurrentUser,
    pub logs_channel: PrivateChannel,
    pub cursed_channel: ChannelId,
    pub db: PgPool,
    pub startup: DateTime<Local>,
    pub threshold: u64,
    pub autoreplies: Mutex<Vec<AutoReply>>,
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
