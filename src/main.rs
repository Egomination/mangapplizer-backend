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
extern crate log;

use actix_web::{
    middleware,
    web,
    App,
    HttpServer,
};

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    std::env::set_var("RUST_BACKTRACE", "1")
    env_logger::init();
    let sys = actix::System::new("mangapplizer_backend");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .data(db_connection::establish_connection())
            .service(
                web::resource("/mangas")
                    .route(web::get().to_async(handlers::mangas::index))
                    .route(web::post().to_async(handlers::mangas::create)),
            )
            .service(
                web::resource("/mangas/{manga_id}")
                    .route(web::get().to_async(handlers::mangas::find)),
            )
            .service(
                web::resource("/insert").route(
                    web::post().to_async(handlers::mangas::insert_chapter),
                ),
            )
    })
    .bind("0.0.0.0:9092")
    .unwrap()
    .start();

    println!("Started!");
    let _ = sys.run();
}
