// 3rd-party imports
use sqlx::PgPool;

use super::models;

/// Insert a book in to the database.
/// On success returns the id of the new row
pub async fn book_create(
    pool: &PgPool,
    book: &models::CreateBookBody,
) -> sqlx::Result<(i64,), sqlx::Error> {
    sqlx::query_as(
        "INSERT INTO book (title, year_published, author_id, rented_to) VALUES (? ? ? ?) RETURNING id",
    )
    .bind(&book.title)
    .bind(&book.year_published)
    .bind(book.author_id)
    .fetch_one(pool)
    .await
}

pub async fn book_list(pool: &PgPool) -> sqlx::Result<Vec<models::Book>> {
    let raw = sqlx::query("SELECT id, title, year_published, author_id FROM book")
        .fetch_all(pool)
        .await?;
    Ok(raw
        .iter()
        .map(|row| models::Book::from_pg_row(row))
        .collect())
}
