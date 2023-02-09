use actix_web::web;

#[derive(serde::Serialize)]
pub struct Book {
    id: i64,
    title: String,
    year_published: String,
    author_id: u32,
    rented_to: Option<String>,
}

impl Book {
    pub fn from_create_book_body(create_book_body: web::Json<CreateBookBody>, id: i64) -> Self {
        Self {
            id,
            title: create_book_body.title.clone(),
            year_published: create_book_body.year_published.clone(),
            author_id: create_book_body.author_id,
            rented_to: create_book_body.rented_to.clone()
        }
    }
}

#[derive(serde::Deserialize)]
pub struct CreateBookBody {
    pub title: String,
    pub year_published: String,
    pub author_id: u32,
    pub rented_to: Option<String>,
}

