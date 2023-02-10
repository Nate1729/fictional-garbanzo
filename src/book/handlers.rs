use actix_web::{web, HttpResponse};

use super::{models, queries};
use crate::AppState;

pub async fn create_book(
    body: web::Json<models::CreateBookBody>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match queries::book_create(&state.sql_client, &body).await {
        Ok(id) => HttpResponse::Created().json(models::Book::from_create_book_body(body, id)),
        Err(_) => HttpResponse::InternalServerError().body("Book could not be created!"),
    }
}

pub async fn list_books(state: web::Data<AppState>) -> web::Json<Vec<models::Book>> {
    let books = match queries::book_list(&state.sql_client).await {
        Ok(b) => b,
        Err(e) => panic!("Error querying for books, Error: {}", e),
    };

    web::Json(books)
}
