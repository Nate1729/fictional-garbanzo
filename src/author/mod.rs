// 3rd Party Imports
use actix_web::web;

pub mod handlers;
pub mod models;
pub mod queries;

pub fn author_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/author")
            .route(web::get().to(handlers::get_authors))
            .route(web::post().to(handlers::create_author)),
    );
}
