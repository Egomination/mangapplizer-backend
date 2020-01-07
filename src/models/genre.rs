use crate::schema::genres;
use diesel::PgConnection;

#[derive(
    Queryable,
    QueryableByName,
    Identifiable,
    Associations,
    Debug,
    Serialize,
    Deserialize,
    AsChangeset,
)]
#[table_name = "genres"]
pub struct Genre {
    pub id:          i64,
    pub genre_name:  String,
    pub description: Option<String>,
}

#[derive(Insertable, Debug, Deserialize, AsChangeset)]
#[table_name = "genres"]
pub struct NewGenre<'a> {
    pub genre_name:  String,
    pub description: &'a str,
}

impl<'a> NewGenre<'a> {
    pub fn create(
        &self,
        connection: &PgConnection,
    ) -> Result<Genre, diesel::result::Error> {
        use crate::schema::genres::columns::genre_name;
        use diesel::ExpressionMethods;
        use diesel::RunQueryDsl;

        diesel::insert_into(genres::table)
            .values(self)
            .on_conflict(genre_name)
            .do_update()
            .set(genres::genre_name.eq(self.genre_name.to_owned()))
            .get_result(connection)
    }

    pub fn insert_genre(
        genre_data: &[String],
        connection: &PgConnection,
    ) -> Vec<i64> {
        use crate::schema::genres::columns::genre_name;
        use diesel::ExpressionMethods;
        use diesel::RunQueryDsl;

        let mut genre_ids = vec![];
        for genre in genre_data.to_owned() {
            let g = Self {
                genre_name:  genre,
                description: "",
            };
            let inserted_genre = diesel::insert_into(genres::table)
                .values(&g)
                .on_conflict(genre_name)
                .do_update()
                .set(genres::genre_name.eq(g.genre_name.to_owned()))
                .returning(genres::id)
                .get_result(connection)
                .unwrap();
            genre_ids.push(inserted_genre);
        }
        genre_ids
    }
}
