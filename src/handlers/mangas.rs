use crate::db_connection::{
    PgPool,
    PgPooledConnection,
};
use crate::models::*;
use actix_web::{
    web,
    HttpRequest,
    HttpResponse,
};
use diesel::PgConnection;
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
pub fn index(
    _req: HttpRequest,
    pool: web::Data<PgPool>,
    manga_search: web::Query<MangaSearch>,
) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    let search = &manga_search.search;
    Ok(HttpResponse::Ok().json(manga::MangaList::list(&pg_pool, search)))
}

// NOTE: Check https://doc.rust-lang.org/rust-by-example/error/iter_result.html
// for more idiomatic implementation for belove function
fn create_staffs(
    manga_id: &uuid::Uuid,
    staffs: &[json_manga::Staff],
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
    relations: &[json_manga::Relation],
    pg_pool: &PgConnection,
) -> Result<uuid::Uuid, diesel::result::Error> {
    for relation in relations.to_owned() {
        let r: relation::NewRelation = relation::NewRelation {
            anilist_id:        relation.anilist_id,
            relationship_type: &relation.relation_type,
            media_type:        &relation.media_type,
            status:            &relation.status,
            title:             &relation.name,
            banner_image:      serde::export::Some::<&str>(&relation.image),
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

fn create_tags(
    manga_id: &uuid::Uuid,
    tags: &[json_manga::Tag],
    pg_pool: &PgConnection,
) -> Result<uuid::Uuid, diesel::result::Error> {
    for tag in tags.to_owned() {
        let t = tag::NewTag {
            tag_name:    &tag.name,
            category:    &tag.category,
            is_spoiler:  tag.is_spoiler,
            description: &tag.description,
        };
        let resp = t.create(&pg_pool);
        match resp {
            Err(e) => return Err(e),
            Ok(response) => {
                ({
                    let tag_list = tag_lists::NewTagList {
                        manga_id: *manga_id,
                        tag_id:   response.id,
                    };
                    let resp: Result<
                        tag_lists::TagList,
                        diesel::result::Error,
                    > = tag_list.create(&pg_pool);
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

fn create_genres(
    manga_id: &uuid::Uuid,
    genres: &[String],
    pg_pool: &PgConnection,
) -> Result<uuid::Uuid, diesel::result::Error> {
    for genre in genres.to_owned() {
        let g = genre::NewGenre {
            genre_name:  &genre,
            description: "",
        };
        let resp = g.create(&pg_pool);
        match resp {
            Err(e) => return Err(e),
            Ok(response) => {
                ({
                    let genre_list = genre_lists::NewGenreList {
                        manga_id: *manga_id,
                        genre_id: response.id,
                    };
                    let resp: Result<
                        genre_lists::GenreList,
                        diesel::result::Error,
                    > = genre_list.create(&pg_pool);
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
        .and_then(|manga| create_staffs(&manga.id, &new_manga.staff, &pg_pool))
        .and_then(|id| create_relations(&id, &new_manga.relations, &pg_pool))
        .and_then(|id| create_genres(&id, &new_manga.genres, &pg_pool))
        .and_then(|id| create_tags(&id, &new_manga.tags, &pg_pool))
        .map(|id| HttpResponse::Ok().json(id))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Chapter {
    manga_name: String,
    chapters:   Vec<Page>,
}

fn print_pages(map: &HashMap<String, String>) {
    for (key, value) in &*map {
        println!("{} / {}", key, value);
    }
    // map.clear();
}
// fn print_pages(map: &HashMap<String, String>) {
//     for (key, value) in &*map {
//         println!("{} / {}", key, value);
//     }
//     // map.clear();
// }

pub fn insert_chapter(
    chapter_data: web::Json<Chapter>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    let search = &chapter_data.manga_name;
    let search_result = manga::MangaList::list(&pg_pool, search);
    chapter_data.chapters.iter().for_each(|m| print_pages(m));
    if search_result.len() > 1 {
        return Err(HttpResponse::InternalServerError().json("hata"));
    }
    Ok(HttpResponse::Ok().json(search_result))
}

pub fn find(
    manga_id: web::Path<String>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    response::Response::full(manga_id.to_string(), &pg_pool)
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
