use actix_web::{
    web,
    HttpRequest,
    HttpResponse,
};

use crate::models::json_manga;

use crate::db_connection::{
    PgPool,
    PgPooledConnection,
};
use crate::models::manga;

fn pg_pool_handler(
    pool: web::Data<PgPool>
) -> Result<PgPooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

// This is calling the list method on ProductList and
// serializing it to a json response
pub fn index(
    _req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Ok(HttpResponse::Ok().json(manga::MangaList::list(&pg_pool)))
}

pub fn create(
    new_manga: web::Json<json_manga::Manga>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    let m = manga::NewManga {
        anilist_id:     new_manga.anilist_id,
        cover_image:    new_manga.cover_image.large.to_owned(),
        banner_image:   new_manga.banner_image.to_owned(),
        start_date:     new_manga.start_date.to_string().to_owned(),
        end_date:       new_manga.end_date.to_string().to_owned(),
        status:         new_manga.status.to_owned(),
        title:          new_manga.manga_name.native.to_owned(),
        description:    new_manga.description.to_owned(),
        total_chapters: new_manga.total_chapters.to_owned(),
        volumes:        new_manga.volumes.to_owned(),
        genres:         new_manga.genres.to_owned(),
        popularity:     new_manga.popularity,
    };
    m.create(&pg_pool)
        .map(|manga| HttpResponse::Ok().json(manga))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
