use actix_web::{web, HttpResponse};

use super::{helpers, models};
use crate::AppState;

pub async fn create_author(
    body: web::Json<models::CreateAuthorBody>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match helpers::author_create(&state.sql_client, &body.name).await {
        Ok(id) => HttpResponse::Created().json(models::AuthorCreateResponse::new(id)),
        Err(_) => HttpResponse::InternalServerError().body("Author not created!"),
    }
}

pub async fn get_authors(state: web::Data<AppState>) -> web::Json<Vec<models::Author>> {
    let authors = helpers::author_list(&state.sql_client).await.unwrap();
    web::Json(authors)
}
