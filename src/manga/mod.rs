mod handler;
pub mod model;
pub(crate) mod service;

use crate::manga::handler::search;
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/manga")
            .service(web::resource("/search").route(web::get().to(search))),
    );
}
