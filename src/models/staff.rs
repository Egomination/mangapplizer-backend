use crate::schema::staffs;
use diesel::PgConnection;

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

impl NewStaff {
    pub fn create(
        &self,
        connection: &PgConnection,
    ) -> Result<Staff, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(staffs::table)
            .values(self)
            .get_result(connection)
    }
}
