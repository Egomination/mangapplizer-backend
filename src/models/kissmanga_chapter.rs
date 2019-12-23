use crate::schema::kissmanga_chapters;
use diesel::PgConnection;

#[derive(
    Queryable,
    QueryableByName,
    Identifiable,
    Associations,
    Debug,
    Serialize,
    Deserialize,
)]
#[table_name = "kissmanga_chapters"]
pub struct KmChapter {
    pub id:          i64,
    pub manga_id:    uuid::Uuid,
    pub source_name: String,
    pub source_type: String,
    pub chapter_no:  i32,
    pub pages:       serde_json::Value,
}

#[derive(Insertable, Debug, Deserialize)]
#[table_name = "kissmanga_chapters"]
pub struct NewKmChapter<'a> {
    pub manga_id:    uuid::Uuid,
    pub source_name: &'a str,
    pub source_type: &'a str,
    pub chapter_no:  i32,
    pub pages:       serde_json::Value,
}

impl<'a> NewKmChapter<'a> {
    pub fn create(
        &self,
        connection: &PgConnection,
    ) -> Result<KmChapter, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(kissmanga_chapters::table)
            .values(self)
            .get_result::<KmChapter>(connection)
    }
}
