use crate::models::json_manga;
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

    pub fn insert_staff(
        staffs_data: &[json_manga::Staff],
        connection: &PgConnection,
    ) -> Vec<uuid::Uuid> {
        use crate::schema::staffs::columns::staff_name;
        use diesel::ExpressionMethods;
        use diesel::RunQueryDsl;

        let mut staff_ids = vec![];
        for staff in staffs_data.to_owned() {
            let s = Self {
                anilist_id:  staff.anilist_id,
                staff_role:  staff.position,
                staff_name:  staff.name,
                image:       staff.picture.large,
                description: staff.description,
            };
            let inserted_staff = diesel::insert_into(staffs::table)
                .values(&s)
                .on_conflict(staff_name)
                .do_update()
                .set(staffs::updated_at.eq(std::time::SystemTime::now()))
                .returning(staffs::id)
                .get_result(connection)
                .unwrap();
            staff_ids.push(inserted_staff);
        }
        staff_ids
    }
}
