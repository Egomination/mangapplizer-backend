use crate::schema::tags;
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
#[table_name = "tags"]
pub struct Tag {
    pub id:          i64,
    pub tag_name:    String,
    pub category:    String,
    pub is_spoiler:  bool,
    pub description: String,
}

#[derive(Insertable, Debug, Deserialize, AsChangeset)]
#[table_name = "tags"]
pub struct NewTag<'a> {
    pub tag_name:    &'a str,
    pub category:    &'a str,
    pub is_spoiler:  bool,
    pub description: &'a str,
}

impl<'a> NewTag<'a> {
    pub fn create(
        &self,
        connection: &PgConnection,
    ) -> Result<Tag, diesel::result::Error> {
        use crate::schema::tags::columns::tag_name;
        use diesel::ExpressionMethods;
        use diesel::RunQueryDsl;

        diesel::insert_into(tags::table)
            .values(self)
            .on_conflict(tag_name)
            .do_update()
            .set(tags::tag_name.eq(self.tag_name))
            .get_result(connection)
    }
}
