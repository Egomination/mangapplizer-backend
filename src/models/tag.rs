use crate::models::json_manga;
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
pub struct NewTag {
    pub tag_name:    String,
    pub category:    String,
    pub is_spoiler:  bool,
    pub description: String,
}

impl NewTag {
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
            .set(tags::tag_name.eq(self.tag_name.to_owned()))
            .get_result(connection)
    }

    pub fn insert_tag(
        tag_data: &[json_manga::Tag],
        connection: &PgConnection,
    ) -> Vec<i64> {
        use crate::schema::tags::columns::tag_name;
        use diesel::ExpressionMethods;
        use diesel::RunQueryDsl;

        let mut tag_ids = vec![];
        for tag in tag_data.to_owned() {
            let t = Self {
                tag_name:    tag.name,
                category:    tag.category,
                is_spoiler:  tag.is_spoiler,
                description: tag.description,
            };
            let inserted_tag = diesel::insert_into(tags::table)
                .values(&t)
                .on_conflict(tag_name)
                .do_update()
                .set(tags::tag_name.eq(t.tag_name.to_owned()))
                .returning(tags::id)
                .get_result(connection)
                .unwrap();
            tag_ids.push(inserted_tag);
        }
        tag_ids
    }
}
