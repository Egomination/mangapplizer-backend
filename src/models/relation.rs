use crate::schema::relations;

#[derive(Queryable, Identifiable, Associations, Debug)]
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
    pub banner_image:      String,
}

#[derive(Insertable, Debug)]
#[table_name = "relations"]
pub struct NewRelation {
    pub media_type:        String,
    pub anilist_id:        i64,
    pub relationship_type: String,
    pub status:            String,
    pub title:             String,
    pub banner_image:      String,
}
