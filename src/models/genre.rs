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
    pub genre_name:  &'a str,
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
            .set(genres::genre_name.eq(self.genre_name))
            .get_result(connection)
    }
}
