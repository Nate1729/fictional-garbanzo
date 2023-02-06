// 3rd Party imports
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{sqlite::Sqlite, Pool, SqlitePool};

// Local imports
mod author;
mod author_handlers;

pub struct AppState {
    sql_client: Pool<Sqlite>,
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
            .configure(author::author_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
