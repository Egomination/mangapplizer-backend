#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate actix;
extern crate actix_web;

pub mod db_connection;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod schema;
// pub mod utils;
