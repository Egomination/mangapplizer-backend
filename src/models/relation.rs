use crate::models::json_manga;
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
pub struct NewRelation {
    pub media_type:        String,
    pub anilist_id:        i64,
    pub relationship_type: String,
    pub status:            String,
    pub title:             String,
    pub banner_image:      Option<String>,
}

impl NewRelation {
    pub fn create(
        &self,
        connection: &PgConnection,
    ) -> Result<Relation, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(relations::table)
            .values(self)
            .get_result(connection)
    }

    pub fn insert_relation(
        relation_data: &[json_manga::Relation],
        connection: &PgConnection,
    ) -> Vec<uuid::Uuid> {
        use diesel::RunQueryDsl;

        let mut relation_ids = vec![];
        for relation in relation_data.to_owned() {
            let r = Self {
                anilist_id:        relation.anilist_id,
                relationship_type: relation.relation_type,
                media_type:        relation.media_type,
                status:            relation.status,
                title:             relation.name,
                banner_image:      serde::export::Some::<String>(
                    relation.image,
                ),
            };

            let inserted_relation = diesel::insert_into(relations::table)
                .values(&r)
                .returning(relations::id)
                .get_result(connection)
                .unwrap();
            relation_ids.push(inserted_relation);
        }
        relation_ids
    }
}
