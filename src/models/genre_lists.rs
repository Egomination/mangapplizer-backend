use crate::models::{
    genre,
    manga,
};
use crate::schema::genres_lists;
use diesel::PgConnection;

#[derive(
    QueryableByName,
    Queryable,
    Identifiable,
    Associations,
    Serialize,
    Deserialize,
    Debug,
)]
#[belongs_to(manga::Manga)]
#[belongs_to(genre::Genre)]
#[table_name = "genres_lists"]
pub struct GenreList {
    pub id:       i64,
    pub manga_id: uuid::Uuid,
    pub genre_id: i64,
}

#[derive(Insertable, Debug)]
#[table_name = "genres_lists"]
pub struct NewGenreList {
    pub manga_id: uuid::Uuid,
    pub genre_id: i64,
}

impl NewGenreList {
    pub fn create(
        &self,
        connection: &PgConnection,
    ) -> Result<GenreList, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(genres_lists::table)
            .values(self)
            .get_result(connection)
    }

    pub fn insert_genre_list(
        manga_id: &uuid::Uuid,
        genre_ids: Vec<i64>,
        connection: &PgConnection,
    ) -> Result<(), diesel::result::Error> {
        use diesel::RunQueryDsl;

        for gid in genre_ids {
            let gl = Self {
                manga_id: *manga_id,
                genre_id: gid,
            };
            diesel::insert_into(genres_lists::table)
                .values(&gl)
                .get_result::<GenreList>(connection)
                .unwrap();
        }
        Ok(())
    }
}
