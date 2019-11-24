use crate::models::{
    manga,
    tag,
};
use crate::schema::tags_lists;
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
#[belongs_to(tag::Tag)]
#[table_name = "tags_lists"]
pub struct TagList {
    pub id:       i64,
    pub manga_id: uuid::Uuid,
    pub tag_id:   i64,
}

#[derive(Insertable, Debug)]
#[table_name = "tags_lists"]
pub struct NewTagList {
    pub manga_id: uuid::Uuid,
    pub tag_id:   i64,
}

impl NewTagList {
    pub fn create(
        &self,
        connection: &PgConnection,
    ) -> Result<TagList, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(tags_lists::table)
            .values(self)
            .get_result(connection)
    }
}
