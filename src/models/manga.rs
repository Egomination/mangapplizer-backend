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
    pub start_date:        &'a str,
    pub end_date:          &'a str,
    pub status:            &'a str,
    pub description:       &'a str,
    pub total_chapters:    Option<&'a str>,
    pub volumes:           Option<&'a str>,
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
}
