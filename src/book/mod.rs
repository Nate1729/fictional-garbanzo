use actix_web::web;

// 3rd-party imports
pub mod handlers;
pub mod models;
pub mod queries;

pub fn book_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/book")
            .route(web::get().to(handlers::get_book))
            .route(web::post().to(handlers::create_book)),
    );
}
