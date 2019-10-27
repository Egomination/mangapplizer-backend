use crate::models::{
    manga,
    relation,
};
use crate::schema::media;

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
