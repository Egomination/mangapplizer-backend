use super::schema::*;
use uuid::Uuid;

// Return type from the db
#[derive(Queryable, Identifiable, Associations, Debug)]
pub struct Manga {
    pub id:           Uuid,
    pub created_at:   Option<std::time::SystemTime>,
    pub updated_at:   Option<std::time::SystemTime>,
    pub deleted_at:   Option<std::time::SystemTime>,
    pub anilist_id:   i64,
    pub cover_image:  String,
    pub banner_image: String,
    pub start_date:   String,
    pub end_date:     String,
    pub status:       String,
    pub title:        String,
}
// Used when new manga is going to be inserted into the database
#[derive(Insertable, Debug)]
#[table_name = "mangas"]
pub struct NewManga {
    pub anilist_id:   i64,
    pub cover_image:  String,
    pub banner_image: String,
    pub start_date:   String,
    pub end_date:     String,
    pub status:       String,
    pub title:        String,
}

#[derive(Queryable, Identifiable, Associations, Debug)]
#[table_name = "relations"]
pub struct Relation {
    pub id:                Uuid,
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

#[derive(Queryable, Identifiable, Associations, Debug)]
pub struct Staff {
    pub id:         Uuid,
    pub created_at: Option<std::time::SystemTime>,
    pub deleted_at: Option<std::time::SystemTime>,
    pub updated_at: Option<std::time::SystemTime>,
    pub anilist_id: i64,
    pub role:       String,
    pub name:       String,
    pub image:      String,
}

#[derive(Insertable, Debug)]
#[table_name = "staffs"]
pub struct NewStaff {
    pub anilist_id: i64,
    pub role:       String,
    pub name:       String,
    pub image:      String,
}

#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(Manga)]
#[belongs_to(Relation)]
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

#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(Manga)]
#[belongs_to(Staff)]
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
