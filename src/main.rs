// 3rd Party imports
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::Deserialize;
use sqlx::{sqlite::Sqlite, sqlite::SqliteQueryResult, Pool, SqlitePool};

// Local imports
mod author;
use author::author_config;

pub struct AppState {
    sql_client: Pool<Sqlite>,
}

#[derive(serde::Serialize, Debug)]
struct Author {
    id: i64,
    name: Option<String>,
}

#[derive(serde::Serialize, Debug)]
struct AuthorListResponse {
    authors: Vec<Author>,
}

async fn add_author(pool: &Pool<Sqlite>, name: &String) -> sqlx::Result<SqliteQueryResult> {
    let mut conn = pool.acquire().await?;
    sqlx::query!(r#"INSERT INTO author (name) VALUES ( ?1 )"#, name)
        .execute(&mut conn)
        .await
}

#[derive(Deserialize)]
struct CreateAuthorBody {
    name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

//#[get("/author/list")]
//async fn author_list(state: web::Data<AppState>) -> impl Responder {
//    let authors = list_author(&state.sql_client).await.unwrap();
//    let response = AuthorListResponse { authors };
//    web::Json(response)
//}

#[post("/author-create")]
async fn author_create(
    body: web::Json<CreateAuthorBody>,
    state: web::Data<AppState>,
) -> impl Responder {
    match add_author(&state.sql_client, &body.name).await {
        Ok(_) => HttpResponse::Ok().body(format!("Author Created!")),
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //std::env::set_var("RUST_LOG", "debug");
    //env_logger::init();
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                sql_client: pool.clone(),
            }))
            .configure(author_config)
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
