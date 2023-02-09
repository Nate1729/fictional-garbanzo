// 3rd-party imports
use sqlx::{Pool, Sqlite};

use super::models;

pub async fn book_create(
    pool: &Pool<Sqlite>,
    book: &models::CreateBookBody,
) -> sqlx::Result<i64> {
    let mut conn = pool.acquire().await?;
    let id = sqlx::query(
        "INSERT INTO book (title, year_published, author_id, rented_to) VALUES (? ? ? ?)",
    )
    .bind(&book.title)
    .bind(&book.year_published)
    .bind(book.author_id)
    .bind(&book.rented_to)
    .execute(&mut conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}
