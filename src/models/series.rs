use crate::models::{
    manga,
    staff,
};
use crate::schema::series;

#[derive(Queryable, Identifiable, Associations, Debug)]
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
