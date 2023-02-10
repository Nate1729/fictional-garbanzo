use actix_web::web;

// 3rd-party imports
pub mod handlers;
pub mod models;
pub mod queries;

pub fn book_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/book")
            .service(web::resource("create").route(web::post().to(handlers::create_book)))
            .service(web::resource("list").route(web::get().to(handlers::list_books))),
    );
}
