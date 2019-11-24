use crate::schema::staffs;
use diesel::PgConnection;

#[derive(
    QueryableByName,
    Queryable,
    Identifiable,
    Associations,
    Serialize,
    Deserialize,
    Debug,
)]
#[table_name = "staffs"]
pub struct Staff {
    pub id:          uuid::Uuid,
    pub created_at:  Option<std::time::SystemTime>,
    pub deleted_at:  Option<std::time::SystemTime>,
    pub updated_at:  Option<std::time::SystemTime>,
    pub anilist_id:  i64,
    pub staff_role:  String,
    pub staff_name:  String,
    pub image:       String,
    pub description: String,
}

#[derive(Insertable, Debug)]
#[table_name = "staffs"]
pub struct NewStaff {
    pub anilist_id:  i64,
    pub staff_role:  String,
    pub staff_name:  String,
    pub image:       String,
    pub description: String,
}

impl NewStaff {
    pub fn create(
        &self,
        connection: &PgConnection,
    ) -> Result<Staff, diesel::result::Error> {
        use crate::schema::staffs::columns::staff_name;
        use diesel::ExpressionMethods;
        use diesel::RunQueryDsl;

        // Checks if there's a staff in the table.
        // if it exists, we just update the updated at table
        // then use the staff uuid for another insertions
        diesel::insert_into(staffs::table)
            .values(self)
            .on_conflict(staff_name)
            .do_update()
            .set(staffs::updated_at.eq(std::time::SystemTime::now()))
            .get_result(connection)
    }
}
