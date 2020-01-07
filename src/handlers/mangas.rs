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
            relationship_type: relation.relation_type,
            media_type:        relation.media_type,
            status:            relation.status,
            title:             relation.name,
            banner_image:      serde::export::Some::<String>(relation.image),
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
            tag_name:    tag.name,
            category:    tag.category,
            is_spoiler:  tag.is_spoiler,
            description: tag.description,
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
            genre_name:  genre,
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

pub async fn create(
    new_manga: web::Json<json_manga::Manga>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;

    let null_msg = "".to_string();

    let m = manga::NewManga {
        anilist_id:        new_manga.anilist_id,
        cover_image:       &new_manga.cover_image.large,
        banner_image:      &new_manga.banner_image,
        start_date:        new_manga.start_date.to_string(),
        end_date:          new_manga.end_date.to_string(),
        status:            &new_manga.status,
        description:       &new_manga.description,
        total_chapters:    serde::export::Some(
            new_manga
                .total_chapters
                .as_ref()
                .unwrap_or(&null_msg)
                .to_string(),
        ),
        volumes:           serde::export::Some(
            new_manga.volumes.as_ref().unwrap_or(&null_msg).to_string(),
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Chapter {
    manga_name: String,
    chapters:   Vec<Page>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryData {
    source_name: String,
    source_type: String,
}

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
    ) + 1;
    chapter_data.chapters.iter().for_each(|c| {
        // I am going to store Page pairs as Json in Postgres
        let chapter_json_data = serde_json::to_value(&c);
        let chapter = kissmanga_chapter::NewKmChapter {
            manga_id:    search_result.0[0].id,
            source_name: &query.source_name,
            source_type: &query.source_type,
            chapter_no:  ch_no,
            pages:       chapter_json_data.unwrap(),
        };

        let result = chapter.create(&pg_pool);
        match result {
            Err(e) => {
                {
                    log::error!("Cannot insert chapter!, {}", e.to_string())
                }
                ()
            }
            Ok(response) => {
                {
                    log::info!("Chapter {:#?} inserted", response);
                    ch_no += 1;
                }
                ()
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

pub fn insert_manga_v2(
    new_manga: web::Json<json_manga::Manga>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    let m: json_manga::Manga = new_manga.clone();
    manga::NewManga::insert_manga(m, &pg_pool)
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
