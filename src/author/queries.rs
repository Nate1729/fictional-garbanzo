// 3rd party imports
use sqlx::PgPool;

// Module imports
use super::models;

pub async fn author_list(pool: &PgPool) -> sqlx::Result<Vec<models::Author>> {
    sqlx::query_as!(models::Author, r"SELECT * FROM author")
        .fetch_all(pool)
        .await
}

pub async fn author_create(
    pool: &PgPool,
    first_name: &str,
    last_name: &str,
) -> sqlx::Result<(i64,), sqlx::Error> {
    sqlx::query_as(
        r#"INSERT INTO author (first_name, last_name)
        VALUES ( $1, $2 )
        RETURNING id"#,
    )
    .bind(first_name)
    .bind(last_name)
    .fetch_one(pool)
    .await
}
