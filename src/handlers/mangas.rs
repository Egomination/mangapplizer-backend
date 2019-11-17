use crate::db_connection::{
    PgPool,
    PgPooledConnection,
};
use crate::models::{
    json_manga,
    manga,
    media,
    relation,
    response,
    series,
    staff,
};
use actix_web::{
    web,
    HttpRequest,
    HttpResponse,
};
use diesel::PgConnection;

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

// TODO: Check https://doc.rust-lang.org/rust-by-example/error/iter_result.html
// for more idiomatic implementation for belove function
fn create_staffs(
    manga_id: &uuid::Uuid,
    staffs: &Vec<json_manga::Staff>,
    pg_pool: &PgConnection,
) -> Result<uuid::Uuid, diesel::result::Error> {
    for staff in staffs.to_owned() {
        let s = staff::NewStaff {
            anilist_id:  staff.anilist_id,
            staff_role:  staff.position,
            staff_name:  staff.name,
            image:       staff.picture.large,
            description: staff.description,
        };
        let resp = s.create(&pg_pool);
        match resp {
            Err(e) => return Err(e),
            Ok(response) => {
                ({
                    let series = series::NewSeries {
                        manga_id: *manga_id,
                        staff_id: response.id,
                    };
                    let resp: Result<series::Series, diesel::result::Error> =
                        series.create(&pg_pool);
                    match resp {
                        Err(e) => return Err(e),
                        Ok(_response) => (),
                    }
                })
            }
        }
    }

    Ok(*manga_id)
}

fn create_relations(
    manga_id: &uuid::Uuid,
    relations: &Vec<json_manga::Relation>,
    pg_pool: &PgConnection,
) -> Result<uuid::Uuid, diesel::result::Error> {
    for relation in relations.to_owned() {
        let r = relation::NewRelation {
            anilist_id:        relation.anilist_id,
            relationship_type: relation.relation_type,
            media_type:        relation.media_type,
            status:            relation.status,
            title:             relation.name,
            banner_image:      relation.image,
        };
        let resp = r.create(&pg_pool);
        match resp {
            Err(e) => return Err(e),
            Ok(response) => {
                ({
                    let m = media::NewMedia {
                        manga_id:    *manga_id,
                        relation_id: response.id,
                    };
                    let response: Result<media::Media, diesel::result::Error> =
                        m.create(&pg_pool);
                    match response {
                        Err(e) => return Err(e),
                        Ok(_response) => (),
                    }
                })
            }
        }
    }
    Ok(*manga_id)
}

pub fn create(
    new_manga: web::Json<json_manga::Manga>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;

    let null_msg = "".to_string();

    let m = manga::NewManga {
        anilist_id:        new_manga.anilist_id,
        cover_image:       &new_manga.cover_image.large,
        banner_image:      &new_manga.banner_image,
        start_date:        &new_manga.start_date.to_string(),
        end_date:          &new_manga.end_date.to_string(),
        status:            &new_manga.status,
        description:       &new_manga.description,
        total_chapters:    serde::export::Some(
            &new_manga.total_chapters.as_ref().unwrap_or(&null_msg),
        ),
        volumes:           serde::export::Some(
            &new_manga.volumes.as_ref().unwrap_or(&null_msg),
        ),
        english_title:     &new_manga.manga_name.english,
        romaji_title:      &new_manga.manga_name.romaji,
        native_title:      &new_manga.manga_name.native,
        cover_extra_large: &new_manga.cover_image.extra_large,
        cover_large:       &new_manga.cover_image.large,
        cover_medium:      &new_manga.cover_image.medium,
        popularity:        new_manga.popularity,
    };

    m.create(&pg_pool)
        .and_then(|r| create_staffs(&r.id, &new_manga.staff, &pg_pool))
        .and_then(|id| create_relations(&id, &new_manga.relations, &pg_pool))
        .map(|id| HttpResponse::Ok().json(id))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))

    // m.create(&pg_pool)
    //     .map(|manga| HttpResponse::Ok().json(manga))
    //     .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn find(
    manga_id: String,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    response::Response::full(manga_id, &pg_pool)
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
