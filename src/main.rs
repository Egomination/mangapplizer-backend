extern crate actix_web;
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

use actix_web::{
    web,
    App,
    HttpRequest,
    HttpResponse,
    HttpServer,
};
// use mangapplizer_backend::test_print_data;
// use mangapplizer_backend::test_get_data_from_db;

fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json("Hello World!")
}

fn main() {
    // let _a = test_print_data();
    // 10931acb-dd12-4f13-ac02-82a3372a7acf
    let my_uuid =
        uuid::Uuid::parse_str("10931acb-dd12-4f13-ac02-82a3372a7acf").unwrap();
    println!("{}", my_uuid.to_urn());

    let _ = HttpServer::new(|| {
        App::new().service(web::resource("/").route(web::get().to_async(index)))
    })
    .bind("0.0.0.0:9092")
    .unwrap()
    .run();

    // test_get_data_from_db(my_uuid);
}
