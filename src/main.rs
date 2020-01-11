extern crate actix_web;

mod cli_args;
mod database;
pub mod db_connection;
pub mod errors;
pub mod handlers;
mod manga;
pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
// extern crate log;

use actix_web::{
    middleware,
    web,
    App,
    HttpServer,
};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();
    // let sys = actix::System::new("mangapplizer_backend");

    let opt = {
        use structopt::StructOpt;
        cli_args::Opt::from_args()
    };

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .data(database::pool::establish_connection(opt.clone()))
            .configure(manga::route)
            .service(
                web::resource("/mangas")
                    .route(web::get().to(handlers::mangas::index))
                    .route(web::post().to(handlers::mangas::insert_manga_v2)),
            )
            .service(
                web::resource("/mangas/{manga_id}")
                    .route(web::get().to(handlers::mangas::find)),
            )
            .service(
                web::resource("/insert")
                    .route(web::post().to(handlers::mangas::insert_chapter)),
            )
    })
    .bind("0.0.0.0:9092")?
    .run()
    .await
}
