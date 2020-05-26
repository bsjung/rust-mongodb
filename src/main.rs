#[macro_use]
extern crate bson;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;

mod jwt;
mod state;
mod routes;
mod config;
mod errors;
mod utils;
mod validate;
mod middleware;

mod database;
mod models;
mod handlers;
mod tests;

use actix_web::{App, HttpServer};
use crate::config::CONFIG;
use crate::state::new_state;
use crate::routes::routes;
use actix_cors::Cors;

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    utils::init_logger();
    let data = new_state::<String>();

    let binding_address = &CONFIG.server;
    HttpServer::new(move|| {
        App::new()
            .wrap(Cors::new().supports_credentials().finish())
            .app_data(data.clone())
            .configure(routes)
    })
    .bind(binding_address)
    .expect(&format!("Can not bind to {}", binding_address) )
    .run()
    .await
}
