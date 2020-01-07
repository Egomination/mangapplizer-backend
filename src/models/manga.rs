use crate::models::{
    genre,
    genre_lists,
    json_manga,
    media,
    relation,
    series,
    staff,
    tag,
    tag_lists,
};
use crate::schema::mangas;
use diesel::PgConnection;

// Return type from the db
#[derive(
    Queryable,
    QueryableByName,
    Identifiable,
    Associations,
    Debug,
    Serialize,
    Deserialize,
    AsChangeset,
    PartialEq,
)]
#[table_name = "mangas"]
pub struct Manga {
    pub id:                uuid::Uuid,
    pub created_at:        Option<std::time::SystemTime>,
    pub updated_at:        Option<std::time::SystemTime>,
    pub deleted_at:        Option<std::time::SystemTime>,
    pub anilist_id:        i64,
    pub cover_image:       String,
    pub banner_image:      String,
    pub start_date:        String,
    pub end_date:          String,
    pub status:            String,
    pub description:       String,
    pub total_chapters:    Option<String>,
    pub volumes:           Option<String>,
    pub english_title:     String,
    pub romaji_title:      String,
    pub native_title:      String,
    pub cover_extra_large: String,
    pub cover_large:       String,
    pub cover_medium:      String,
    pub popularity:        i64,
}

type MangaColumns = (
    mangas::id,
    mangas::created_at,
    mangas::updated_at,
    mangas::deleted_at,
    mangas::anilist_id,
    mangas::cover_image,
    mangas::banner_image,
    mangas::start_date,
    mangas::end_date,
    mangas::status,
    mangas::description,
    mangas::total_chapters,
    mangas::volumes,
    mangas::english_title,
    mangas::romaji_title,
    mangas::native_title,
    mangas::cover_extra_large,
    mangas::cover_large,
    mangas::cover_medium,
    mangas::popularity,
);

pub const MANGAS_COLUMNS: MangaColumns = (
    mangas::id,
    mangas::created_at,
    mangas::updated_at,
    mangas::deleted_at,
    mangas::anilist_id,
    mangas::cover_image,
    mangas::banner_image,
    mangas::start_date,
    mangas::end_date,
    mangas::status,
    mangas::description,
    mangas::total_chapters,
    mangas::volumes,
    mangas::english_title,
    mangas::romaji_title,
    mangas::native_title,
    mangas::cover_extra_large,
    mangas::cover_large,
    mangas::cover_medium,
    mangas::popularity,
);

// Used when new manga is going to be inserted into the database
#[derive(Insertable, Debug, Deserialize, AsChangeset, PartialEq)]
#[table_name = "mangas"]
pub struct NewManga<'a> {
    pub anilist_id:        i64,
    pub cover_image:       &'a str,
    pub banner_image:      &'a str,
    pub start_date:        String,
    pub end_date:          String,
    pub status:            &'a str,
    pub description:       &'a str,
    pub total_chapters:    Option<String>,
    pub volumes:           Option<String>,
    pub english_title:     &'a str,
    pub romaji_title:      &'a str,
    pub native_title:      &'a str,
    pub cover_extra_large: &'a str,
    pub cover_large:       &'a str,
    pub cover_medium:      &'a str,
    pub popularity:        i64,
}

#[derive(Serialize, Deserialize)]
pub struct MangaList(pub Vec<Manga>);

impl MangaList {
    pub fn list(
        connection: &PgConnection,
        search: &str,
    ) -> Self {
        use crate::schema;
        use crate::schema::mangas::dsl::*;
        use diesel::pg::Pg;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use diesel_full_text_search::{
            plainto_tsquery,
            TsVectorExtensions,
        };

        let mut query = schema::mangas::table.into_boxed::<Pg>();

        if !search.is_empty() {
            query = query
                .filter(text_searchable_mangas.matches(plainto_tsquery(search)))
        }

        let result = query
            .select(MANGAS_COLUMNS)
            .load::<Manga>(connection)
            .expect("Error Searching Manga");
        MangaList(result)
    }

    pub fn len(&self) -> usize {
        println!("{}", self.0.iter().len());
        self.0.iter().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a> NewManga<'a> {
    pub fn create(
        &self,
        connection: &PgConnection,
    ) -> Result<Manga, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(mangas::table)
            .values(self)
            .returning(MANGAS_COLUMNS)
            .get_result::<Manga>(connection)
    }

    pub fn insert_manga(
        manga_data: json_manga::Manga,
        connection: &PgConnection,
    ) -> Result<Manga, diesel::result::Error> {
        use diesel::Connection;
        use diesel::RunQueryDsl;

        connection.transaction(|| {
            let m = Self {
                anilist_id:        manga_data.anilist_id,
                cover_image:       &manga_data.cover_image.large,
                banner_image:      &manga_data.banner_image,
                start_date:        manga_data.start_date.to_string(),
                end_date:          manga_data.end_date.to_string(),
                status:            &manga_data.status,
                description:       &manga_data.description,
                total_chapters:    serde::export::Some(
                    manga_data
                        .total_chapters
                        .as_ref()
                        .unwrap_or(&"Null".to_string())
                        .to_string(),
                ),
                volumes:           serde::export::Some(
                    manga_data
                        .volumes
                        .as_ref()
                        .unwrap_or(&"Null".to_string())
                        .to_string(),
                ),
                english_title:     &manga_data.manga_name.english,
                romaji_title:      &manga_data.manga_name.romaji,
                native_title:      &manga_data.manga_name.native,
                cover_extra_large: &manga_data.cover_image.extra_large,
                cover_large:       &manga_data.cover_image.large,
                cover_medium:      &manga_data.cover_image.medium,
                popularity:        manga_data.popularity,
            };

            let manga = diesel::insert_into(mangas::table)
                .values(&m)
                .returning(MANGAS_COLUMNS)
                .get_result::<Manga>(connection)?;

            let staff_ids =
                staff::NewStaff::insert_staff(&manga_data.staff, &connection);

            let series = series::NewSeries::insert_series(
                &manga.id,
                staff_ids,
                &connection,
            );
            if series.is_err() {
                panic!("Cannot insert Series!");
            }

            let relation_ids = relation::NewRelation::insert_relation(
                &manga_data.relations,
                &connection,
            );

            let media = media::NewMedia::insert_media(
                &manga.id,
                relation_ids,
                &connection,
            );

            if media.is_err() {
                panic!("Cannot insert Media!");
            }

            let genre_list =
                genre::NewGenre::insert_genre(&manga_data.genres, &connection);

            let genre_list = genre_lists::NewGenreList::insert_genre_list(
                &manga.id,
                genre_list,
                &connection,
            );

            if genre_list.is_err() {
                panic!("Cannot insert Genre List!");
            }

            let tag_ids =
                tag::NewTag::insert_tag(&manga_data.tags, &connection);

            let tag_list = tag_lists::NewTagList::insert_tag_list(
                &manga.id,
                tag_ids,
                &connection,
            );

            if tag_list.is_err() {
                panic!("Cannot insert Tag list!!");
            }

            Ok(manga)
        })
    }
}
