extern crate mangapplizer_backend;

pub mod db_connection;
pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
// use mangapplizer_backend::test_print_data;
// use mangapplizer_backend::test_get_data_from_db;

fn main() {
    // let _a = test_print_data();
    // 10931acb-dd12-4f13-ac02-82a3372a7acf
    let my_uuid =
        uuid::Uuid::parse_str("10931acb-dd12-4f13-ac02-82a3372a7acf").unwrap();
    println!("{}", my_uuid.to_urn());

    // test_get_data_from_db(my_uuid);
}
