use sqlx::{postgres::PgRow, Row};

#[derive(serde::Serialize)]
pub struct Author {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
}

impl Author {
    pub fn from_pg_row(row: &PgRow) -> Self {
        Self {
            id: row.get("id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct CreateAuthorBody {
    pub first_name: String,
    pub last_name: String,
}

#[derive(serde::Serialize)]
pub struct AuthorCreateResponse {
    id: i64,
}

impl AuthorCreateResponse {
    pub fn new(id: i64) -> Self {
        AuthorCreateResponse { id }
    }
}
