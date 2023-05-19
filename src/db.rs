use crate::globals::THRESHOLD;
use crate::globals::DB_URL;

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
            post_id VARCHAR(32) NOT NULL,
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

    Ok(msg.msg_id != "")
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

pub async fn set(id: String, post_id: String, link: String, count: u8) -> Result<(), sqlx::Error> {
    let db = connect().await?;

    sqlx::query(
        "replace into board
        values (?, ?, ?, ?)",
    )
    .bind(id)
    .bind(post_id)
    .bind(link)
    .bind(count)
    .execute(&db)
    .await?;

    Ok(())
}

#[derive(FromRow, Debug, Default)]
pub struct BoardEntry {
    pub link: String,
    pub count: u8,
}

pub async fn list() -> Result<Vec<BoardEntry>, sqlx::Error> {
    let db = connect().await?;

    let entries = sqlx::query_as::<_, BoardEntry>(
        "select link, moyai_count
        from board
        order by moyai_count desc
        limit 10",
    )
    .fetch_all(&db)
    .await?;

    Ok(dbg!(entries))
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

    sqlx::query("delete from board where moyai_count = 0")
        .execute(&db)
        .await?;

    let out: Vec<u64> = result
        .iter()
        .map(|x| x.post_id.parse::<u64>().expect("guh"))
        .collect();

    Ok(out)
}
