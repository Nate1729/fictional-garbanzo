use actix_web::web;
use sqlx::{postgres::PgRow, Row};

#[derive(serde::Serialize)]
pub struct Book {
    pub id: i64,
    pub title: String,
    pub year_published: String,
    pub author_id: i64,
}

impl Book {
    pub fn from_create_book_body(create_book_body: web::Json<CreateBookBody>, id: i64) -> Self {
        Self {
            id,
            title: create_book_body.title.clone(),
            year_published: create_book_body.year_published.clone(),
            author_id: create_book_body.author_id,
        }
    }

    pub fn from_pg_row(row: &PgRow) -> Self {
        Self {
            id: row.get("id"),
            title: row.get("title"),
            year_published: row.get("year_published"),
            author_id: row.get("author_id"),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct CreateBookBody {
    pub title: String,
    pub year_published: String,
    pub author_id: i64,
}
