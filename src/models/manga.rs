use crate::schema::mangas;

// Return type from the db
#[derive(Queryable, Identifiable, Associations, Debug)]
pub struct Manga {
    pub id:           uuid::Uuid,
    pub created_at:   Option<std::time::SystemTime>,
    pub updated_at:   Option<std::time::SystemTime>,
    pub deleted_at:   Option<std::time::SystemTime>,
    pub anilist_id:   i64,
    pub cover_image:  String,
    pub banner_image: String,
    pub start_date:   String,
    pub end_date:     String,
    pub status:       String,
    pub title:        String,
}
// Used when new manga is going to be inserted into the database
#[derive(Insertable, Debug)]
#[table_name = "mangas"]
pub struct NewManga {
    pub anilist_id:   i64,
    pub cover_image:  String,
    pub banner_image: String,
    pub start_date:   String,
    pub end_date:     String,
    pub status:       String,
    pub title:        String,
}
