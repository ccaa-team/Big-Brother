use crate::globals::DB_URL;
use crate::globals::THRESHOLD;

use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool};

async fn create_db() {
    match Sqlite::create_database(DB_URL).await {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
    .unwrap();
}

async fn connect() -> Result<SqlitePool, sqlx::Error> {
    SqlitePool::connect(DB_URL).await
}

pub async fn init() -> Result<(), sqlx::Error> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        create_db().await;
    };

    let db = connect().await?;

    let _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS board (
            msg_id VARCHAR(32) NOT NULL UNIQUE,
            channel_id VARCHAR(32) NOT NULL,
            post_id VARCHAR(32) NOT NULL UNIQUE,
            link VARCHAR(88) NOT NULL,
            moyai_count INT(8) NOT NULL
        );",
    )
    .execute(&db)
    .await?;

    Ok(())
}

#[derive(FromRow, Debug, Default)]
pub struct Message {
    pub msg_id: String,
    pub channel_id: String,
    pub post_id: String,
    pub link: String,
    pub moyai_count: u8,
}

pub async fn exists(id: u64) -> Result<bool, sqlx::Error> {
    let db = connect().await?;
    let id = id.to_string();

    let msg = sqlx::query_as::<_, Message>(
        "select *
        from board
        where msg_id = ?",
    )
    .bind(id)
    .fetch_one(&db)
    .await
    .unwrap_or(Message::default());

    Ok(!msg.msg_id.is_empty())
}

pub async fn get(id: u64) -> Result<Message, sqlx::Error> {
    let db = connect().await?;
    let msg = sqlx::query_as::<_, Message>(
        "select *
        from board
        where msg_id = ?",
    )
    .bind(id.to_string())
    .fetch_one(&db)
    .await
    .unwrap_or(Message::default());

    Ok(msg)
}

pub async fn set(
    msg_id: String,
    channel_id: String,
    post_id: String,
    link: String,
    count: u8,
) -> Result<(), sqlx::Error> {
    let db = connect().await?;

    sqlx::query(
        "replace into board
        values (?, ?, ?, ?, ?)",
    )
    .bind(msg_id)
    .bind(channel_id)
    .bind(post_id)
    .bind(link)
    .bind(count)
    .execute(&db)
    .await?;

    Ok(())
}

#[derive(FromRow, Debug, Default)]
pub struct DBEntry {
    pub link: String,
    pub msg_id: String,
    pub channel_id: String,
    pub moyai_count: u8,
}

pub async fn list() -> Result<Vec<DBEntry>, sqlx::Error> {
    let db = connect().await?;

    let entries = sqlx::query_as::<_, DBEntry>(
        "select link, moyai_count, msg_id, channel_id
        from board
        order by moyai_count desc
        limit 10",
    )
    .fetch_all(&db)
    .await?;

    Ok(entries)
}

pub async fn clean() -> Result<Vec<u64>, sqlx::Error> {
    let db = connect().await?;

    let result = sqlx::query_as::<_, Message>(&format!(
        "SELECT *
        FROM board
        WHERE moyai_count < {THRESHOLD}"
    ))
    .fetch_all(&db)
    .await?;

    let out: Vec<u64> = dbg!(result)
        .iter()
        .map(|x| dbg!(x).post_id.parse::<u64>().unwrap_or(0))
        .collect();

    sqlx::query("delete from board where moyai_count = 0")
        .execute(&db)
        .await?;

    Ok(out)
}
