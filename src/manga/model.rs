use crate::schema::*;
use std::time::SystemTime;
use uuid::Uuid;

/// Manga struct that is returned from the database. Manga has multiple parts
/// such as Relationship, Staff, Tags, Genres and so on.
#[derive(
    Queryable,
    QueryableByName,
    Identifiable,
    Debug,
    Deserialize,
    AsChangeset,
    PartialEq,
    Clone,
)]
#[table_name = "mangas"]
pub struct Manga {
    pub id:                Uuid,
    pub created_at:        Option<SystemTime>,
    pub updated_at:        Option<SystemTime>,
    pub deleted_at:        Option<SystemTime>,
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

/// Manga struct that is going to be inserted into the database.
#[derive(Insertable, Debug, Deserialize, AsChangeset, PartialEq, Clone)]
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
    /// Extra large size of the cover image.
    pub cover_extra_large: &'a str,
    /// large size of the cover images.
    pub cover_large:       &'a str,
    /// Normal size of the cover image.
    pub cover_medium:      &'a str,
    pub popularity:        i64,
}

/// Type for the search result. All of the manga search operations will return
/// to the `MangList` type
#[derive(Serialize, Deserialize)]
pub struct MangaList(pub Vec<Manga>);

impl MangaList {
    pub fn len(&self) -> usize {
        self.0.iter().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
