use crate::schema::mangas;
use diesel::PgConnection;

// Return type from the db
#[derive(
    Queryable, Identifiable, Associations, Debug, Serialize, Deserialize,
)]
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
#[derive(Insertable, Debug, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct MangaList(pub Vec<Manga>);

impl MangaList {
    pub fn list(connection: &PgConnection) -> Self {
        use crate::schema::mangas::dsl::*;
        // use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        let result = mangas
            .load::<Manga>(connection)
            .expect("Error loading mangas");

        MangaList(result)
    }
}

impl NewManga {
    pub fn create(
        &self,
        connection: &PgConnection,
    ) -> Result<Manga, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(mangas::table)
            .values(self)
            .get_result(connection)
    }
}
