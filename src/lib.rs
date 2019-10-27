#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use crate::models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connection to {}", database_url))
}

fn insert_test_data(conn: &PgConnection) -> manga::Manga {
    let manga = manga::NewManga {
        anilist_id:   1,
        cover_image:  "some bullshit".to_string(),
        banner_image: "another_bullshit".to_string(),
        start_date:   "00.00.00".to_string(),
        end_date:     "11.11.11".to_string(),
        status:       "finished".to_string(),
        title:        "ehe".to_string(),
    };

    let staff = staff::NewStaff {
        anilist_id: 1,
        image:      "some-link.com".to_string(),
        name:       "Onez".to_string(),
        role:       "Author".to_string(),
    };

    let relation = relation::NewRelation {
        media_type:        "Anime".to_string(),
        anilist_id:        111,
        status:            "On-going".to_string(),
        title:             "Ehe on roll".to_string(),
        relationship_type: "Adaptation".to_string(),
        banner_image:      "some-link-again.com".to_string(),
    };

    let m: manga::Manga = diesel::insert_into(schema::mangas::table)
        .values(&manga)
        .get_result(conn)
        .expect("Error cannot insert manga");

    let r: relation::Relation = diesel::insert_into(schema::relations::table)
        .values(&relation)
        .get_result(conn)
        .expect("Error cannot insert relation");

    let s: staff::Staff = diesel::insert_into(schema::staffs::table)
        .values(&staff)
        .get_result(conn)
        .expect("Error cannot insert relation");

    // Do the association !! This is the crucial part.

    let m_r = media::NewMedia {
        manga_id:    m.id,
        relation_id: r.id,
    };

    let m_s = series::NewSeries {
        manga_id: m.id,
        staff_id: s.id,
    };

    diesel::insert_into(schema::media::table)
        .values(&m_r)
        .execute(conn)
        .expect("error associating while manga and relation");

    diesel::insert_into(schema::series::table)
        .values(&m_s)
        .execute(conn)
        .expect("error associating while manga and staff");

    m
}

fn get_full_relation(
    manga: &manga::Manga,
    conn: &PgConnection,
) -> std::vec::Vec<staff::Staff> {
    use diesel::pg::expression::dsl::any;

    let manga_staff =
        series::Series::belonging_to(manga).select(schema::series::staff_id);
    schema::staffs::table
        .filter(schema::staffs::id.eq(any(manga_staff)))
        .load::<staff::Staff>(conn)
        .expect("Could not load tags")
}

pub fn test_print_data() {
    let conn = establish_connection();
    let a = insert_test_data(&conn);
    let b = get_full_relation(&a, &conn);
    println!("{:#?}", b);
}
