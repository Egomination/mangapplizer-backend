use crate::db_connection::{PgPool, PgPooledConnection};
use crate::models::*;
use actix_web::{web, HttpRequest, HttpResponse};

use log;
use std::collections::HashMap;

/// Page is the chapter pages
/// [
///     {
///         "0": "string",
///         "1": "str",
///     },
///     {
///         "0": "string",
///     },
/// ]
type Page = HashMap<String, String>;

// TODO:
// Create struct that has genre: Genre, manga: Manga ... field. Pass them into
// the Manga create function and move all of the logic inside the model.

fn pg_pool_handler(
    pool: web::Data<PgPool>
) -> Result<PgPooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Chapter {
    manga_name: String,
    chapters: Vec<Page>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryData {
    source_name: String,
    source_type: String,
}

// TODO: Move insertion logic in kiss_manga.rs
//       Also consider implementing Manga trait which will have
//       insert common function.
pub async fn insert_chapter(
    chapter_data: web::Json<Chapter>,
    query: web::Query<QueryData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    let search = &chapter_data.manga_name;
    let search_result = manga::MangaList::list(&pg_pool, search);

    // Early return if result is not singular
    if search_result.len() > 1 || search_result.is_empty() {
        return Err(HttpResponse::InternalServerError().json("hata"));
    }
    // chapter_data.chapters.iter().for_each(|m| print_pages(m));
    let mut ch_no = kissmanga_chapter::NewKmChapter::latest(
        search_result.0[0].id,
        &pg_pool,
    );
    ch_no += 1;
    chapter_data.chapters.iter().for_each(|c| {
        // I am going to store Page pairs as Json in Postgres
        let chapter_json_data = serde_json::to_value(&c);
        let chapter = kissmanga_chapter::NewKmChapter {
            manga_id: search_result.0[0].id,
            source_name: &query.source_name,
            source_type: &query.source_type,
            chapter_no: ch_no,
            pages: chapter_json_data.unwrap(),
        };

        let result = chapter.create(&pg_pool);
        match result {
            Err(e) => log::error!("Cannot insert chapter!, {}", e.to_string()),
            Ok(response) => {
                log::info!("Chapter {:#?} inserted", response);
                ch_no += 1;
            }
        }
    });
    Ok(HttpResponse::Ok().json("Chapters inserted!"))
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
