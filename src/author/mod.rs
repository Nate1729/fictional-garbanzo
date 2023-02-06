// 3rd Party Imports
use actix_web::web;

pub mod endpoints;
pub mod helpers;
pub mod models;

pub fn author_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/author")
            .service(endpoints::get_author_list)
            .service(endpoints::post_author_create),
    );
}
