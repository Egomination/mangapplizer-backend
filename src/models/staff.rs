use crate::schema::staffs;

#[derive(Queryable, Identifiable, Associations, Debug)]
pub struct Staff {
    pub id:         uuid::Uuid,
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
