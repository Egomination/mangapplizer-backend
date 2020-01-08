use crate::models::{
    manga,
    relation,
};
use crate::schema::media;
use diesel::PgConnection;

#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(manga::Manga)]
#[belongs_to(relation::Relation)]
#[table_name = "media"]
pub struct Media {
    pub id:          i64,
    pub manga_id:    uuid::Uuid,
    pub relation_id: uuid::Uuid,
}

#[derive(Insertable, Debug)]
#[table_name = "media"]
pub struct NewMedia {
    pub manga_id:    uuid::Uuid,
    pub relation_id: uuid::Uuid,
}

impl NewMedia {
    pub fn create(
        &self,
        connection: &PgConnection,
    ) -> Result<Media, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(media::table)
            .values(self)
            .get_result(connection)
    }

    pub fn insert_media(
        manga_id: &uuid::Uuid,
        relation_ids: Vec<uuid::Uuid>,
        connection: &PgConnection,
    ) -> Result<(), diesel::result::Error> {
        use diesel::RunQueryDsl;

        for rid in relation_ids {
            let r = Self {
                manga_id:    *manga_id,
                relation_id: rid,
            };
            diesel::insert_into(media::table)
                .values(&r)
                .get_result::<Media>(connection)
                .unwrap();
        }
        Ok(())
    }
}
