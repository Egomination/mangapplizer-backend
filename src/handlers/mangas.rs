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

use diesel::PgConnection;

use crate::models::{
    manga,
    series,
    staff,
};

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

fn handle_bridge_tables(
    manga_id: &uuid::Uuid,
    staff_ids: &Vec<uuid::Uuid>,
    pg_pool: &PgConnection,
) -> Result<uuid::Uuid, diesel::result::Error> {
    for staff_id in staff_ids.to_owned().into_iter() {
        let series = series::NewSeries {
            manga_id: *manga_id,
            staff_id: staff_id,
        };
        let response = series.create(&pg_pool);
        match response {
            Err(e) => return Err(e),
            Ok(_response) => (),
        }
    }

    Ok(*manga_id)
}

pub fn create(
    new_manga: web::Json<json_manga::Manga>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    let mut staff_ids = vec![];

    let m = manga::NewManga {
        anilist_id:     new_manga.anilist_id,
        cover_image:    new_manga.cover_image.large.to_owned(),
        banner_image:   new_manga.banner_image.to_owned(),
        start_date:     new_manga.start_date.to_owned().to_string(),
        end_date:       new_manga.end_date.to_owned().to_string(),
        status:         new_manga.status.to_owned(),
        title:          new_manga.manga_name.native.to_owned(),
        description:    new_manga.description.to_owned(),
        total_chapters: new_manga.total_chapters.to_owned(),
        volumes:        new_manga.volumes.to_owned(),
        genres:         new_manga.genres.to_owned(),
        popularity:     new_manga.popularity,
    };

    for staff in new_manga.staff.to_owned().into_iter() {
        let s = staff::NewStaff {
            anilist_id: staff.anilist_id,
            role:       staff.position,
            name:       staff.name,
            image:      staff.picture.large,
        };
        let resp: staff::Staff = s.create(&pg_pool).unwrap();
        staff_ids.push(resp.id)
    }

    let manga_response: manga::Manga = m.create(&pg_pool).unwrap();

    println!("{:#?}", staff_ids);
    handle_bridge_tables(&manga_response.id, &staff_ids, &pg_pool)
        .map(|id| HttpResponse::Ok().json(id))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))

    // m.create(&pg_pool)
    //     .map(|manga| HttpResponse::Ok().json(manga))
    //     .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
