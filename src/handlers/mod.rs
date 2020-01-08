// #[macro_use]
// pub mod function_handler;
pub mod mangas;

use actix_web::{
    web,
    HttpResponse,
    //     Result,
};

use crate::db_connection::{
    PgPool,
    PgPooledConnection,
};

pub fn pg_pool_handler(
    pool: web::Data<PgPool>
) -> Result<PgPooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
