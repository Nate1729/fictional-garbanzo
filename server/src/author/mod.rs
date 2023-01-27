// 3rd Party Imports
use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::{Pool, Sqlite};

// Local Imports
use super::AppState;

pub fn author_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/author")
            .service(get_author_list)
            .service(post_author_create),
    );
}

#[derive(serde::Serialize)]
struct Author {
    id: i64,
    name: Option<String>,
}

async fn author_list(pool: &Pool<Sqlite>) -> sqlx::Result<Vec<Author>> {
    let mut conn = pool.acquire().await?;
    sqlx::query_as!(Author, r"SELECT * FROM author")
        .fetch_all(&mut conn)
        .await
}

#[get("/list")]
async fn get_author_list(state: web::Data<AppState>) -> web::Json<Vec<Author>> {
    let authors = author_list(&state.sql_client).await.unwrap();
    web::Json(authors)
}

async fn author_create(pool: &Pool<Sqlite>, name: &String) -> sqlx::Result<i64> {
    let mut conn = pool.acquire().await?;
    let id = sqlx::query!(r#"INSERT INTO author (name) VALUES ( ?1 )"#, name)
        .execute(&mut conn)
        .await?
        .last_insert_rowid();

    Ok(id)
}

#[derive(serde::Deserialize)]
struct CreateAuthorBody {
    name: String,
}

#[derive(serde::Serialize)]
struct AuthorCreateResponse {
    id: i64,
}

#[post("/create")]
async fn post_author_create(
    body: web::Json<CreateAuthorBody>,
    state: web::Data<AppState>,
) -> impl Responder {
    match author_create(&state.sql_client, &body.name).await {
        Ok(id) => HttpResponse::Created().json(AuthorCreateResponse { id }),
        Err(_) => HttpResponse::InternalServerError().body("Author not created!"),
    }
}
