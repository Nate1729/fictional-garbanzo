// 3rd party imports
use sqlx::{Pool, Sqlite};

// Module imports
use super::models;

pub async fn author_list(pool: &Pool<Sqlite>) -> sqlx::Result<Vec<models::Author>> {
    let mut conn = pool.acquire().await?;
    sqlx::query_as!(models::Author, r"SELECT * FROM author")
        .fetch_all(&mut conn)
        .await
}

pub async fn author_create(pool: &Pool<Sqlite>, name: &String) -> sqlx::Result<i64> {
    let mut conn = pool.acquire().await?;
    let id = sqlx::query!(r#"INSERT INTO author (name) VALUES ( ?1 )"#, name)
        .execute(&mut conn)
        .await?
        .last_insert_rowid();

    Ok(id)
}
