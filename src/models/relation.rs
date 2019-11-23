use crate::schema::relations;
use diesel::PgConnection;

#[derive(Serialize, Queryable, Identifiable, Associations, Debug)]
#[table_name = "relations"]
pub struct Relation {
    pub id:                uuid::Uuid,
    pub created_at:        Option<std::time::SystemTime>,
    pub updated_at:        Option<std::time::SystemTime>,
    pub deleted_at:        Option<std::time::SystemTime>,
    pub media_type:        String,
    pub anilist_id:        i64,
    pub relationship_type: String,
    pub status:            String,
    pub title:             String,
    pub banner_image:      Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "relations"]
pub struct NewRelation<'a> {
    pub media_type:        &'a str,
    pub anilist_id:        i64,
    pub relationship_type: &'a str,
    pub status:            &'a str,
    pub title:             &'a str,
    pub banner_image:      Option<&'a str>,
}

impl<'a> NewRelation<'a> {
    pub fn create(
        &self,
        connection: &PgConnection,
    ) -> Result<Relation, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(relations::table)
            .values(self)
            .get_result(connection)
    }
}
