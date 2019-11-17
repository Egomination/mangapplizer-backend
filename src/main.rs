extern crate actix_web;
extern crate mangapplizer_backend;

pub mod db_connection;
pub mod handlers;
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
    HttpServer,
};
// use mangapplizer_backend::test_print_data;
// use mangapplizer_backend::test_get_data_from_db;

fn main() {
    let sys = actix::System::new("mangapplizer_backend");

    HttpServer::new(|| {
        App::new()
            .data(db_connection::establish_connection())
            .service(
                web::resource("/mangas")
                    .route(web::get().to_async(handlers::mangas::find))
                    .route(web::post().to_async(handlers::mangas::create)),
            )
    })
    .bind("0.0.0.0:9092")
    .unwrap()
    .start();

    // HttpServer::new(||  {
    //     App::new().service(
    //         web::resource("/mangas")
    //             .route(web::get().to_async(handlers::mangas::index)),
    //     )
    // })
    // .bind("0.0.0.0:9092")
    // .unwrap()
    // .start();

    println!("Started!");
    let _ = sys.run();
}
