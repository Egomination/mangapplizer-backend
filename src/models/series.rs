use crate::models::{
    manga,
    staff,
};
use crate::schema::series;
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
#[belongs_to(staff::Staff)]
#[table_name = "series"]
pub struct Series {
    pub id:       i64,
    pub manga_id: uuid::Uuid,
    pub staff_id: uuid::Uuid,
}

#[derive(Insertable, Debug)]
#[table_name = "series"]
pub struct NewSeries {
    pub manga_id: uuid::Uuid,
    pub staff_id: uuid::Uuid,
}

impl NewSeries {
    pub fn create(
        &self,
        connection: &PgConnection,
    ) -> Result<Series, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(series::table)
            .values(self)
            .get_result(connection)
    }
}
