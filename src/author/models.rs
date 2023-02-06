#[derive(serde::Serialize)]
pub struct Author {
    pub id: i64,
    pub name: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct CreateAuthorBody {
    pub name: String,
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
