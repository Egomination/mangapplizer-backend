use crate::database::Pool;
use crate::errors::MangapplizerError;
// use crate::manga::model::
use crate::manga::service as manga;
use actix_web::{
    web,
    HttpRequest,
    HttpResponse,
};

pub async fn search(
    search: web::Query<manga::SearchQuery>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MangapplizerError> {
    manga::search(search.into_inner(), pool)
        .map(|res| HttpResponse::Ok().json(&res))
}
