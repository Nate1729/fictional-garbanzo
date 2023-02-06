// 3rd Party Imports
use actix_web::web;

pub mod handlers;
pub mod helpers;
pub mod models;

pub fn author_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/author")
            .service(web::resource("list").route(web::get().to(handlers::get_authors)))
            .service(web::resource("create").route(web::post().to(handlers::create_author))),
    );
}
