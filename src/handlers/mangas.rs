use crate::db_connection::PgPool;
use crate::models::*;
// use actix::prelude::Future;
use actix_web::{
    web,
    HttpRequest,
    HttpResponse,
};

// use log;

use crate::handlers::pg_pool_handler;

#[derive(Deserialize)]
pub struct MangaSearch {
    pub search: String,
}

// This is calling the list method on ProductList and
// serializing it to a json response
pub async fn index(
    _req: HttpRequest,
    pool: web::Data<PgPool>,
    manga_search: web::Query<MangaSearch>,
) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    let search = &manga_search.search;
    Ok(HttpResponse::Ok().json(manga::MangaList::list(&pg_pool, search)))
}

// TODO: Make this generic
pub async fn insert_chapter(
    chapter_data: web::Json<kissmanga_chapter::Chapter>,
    query: web::Query<kissmanga_chapter::QueryData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    let chapters: kissmanga_chapter::Chapter = chapter_data.clone();
    let query: kissmanga_chapter::QueryData = query.clone();
    kissmanga_chapter::NewKmChapter::insert_chapter(&chapters, &query, &pg_pool)
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub async fn find(
    manga_id: web::Path<String>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    response::Response::full(manga_id.to_string(), &pg_pool)
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub async fn insert_manga_v2(
    new_manga: web::Json<json_manga::Manga>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    let m: json_manga::Manga = new_manga.clone();
    manga::NewManga::insert_manga(m, &pg_pool)
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
