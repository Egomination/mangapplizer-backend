// #[macro_use]
// pub mod function_handler;
pub mod mangas;

use actix_web::{
    web,
    HttpResponse,
    //     Result,
};

use crate::database::{
    Pool,
    PooledConnection,
};

pub fn pg_pool_handler(
    pool: web::Data<Pool>
) -> Result<PooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
