// 3rd party imports
use actix_web::{get, post, web, HttpResponse, Responder};

use crate::AppState;

use super::{helpers, models};

#[get("/list")]
pub async fn get_author_list(state: web::Data<AppState>) -> web::Json<Vec<models::Author>> {
    let authors = helpers::author_list(&state.sql_client).await.unwrap();
    web::Json(authors)
}

#[post("/create")]
pub async fn post_author_create(
    body: web::Json<models::CreateAuthorBody>,
    state: web::Data<AppState>,
) -> impl Responder {
    match helpers::author_create(&state.sql_client, &body.name).await {
        Ok(id) => HttpResponse::Created().json(models::AuthorCreateResponse::new(id)),
        Err(_) => HttpResponse::InternalServerError().body("Author not created!"),
    }
}
