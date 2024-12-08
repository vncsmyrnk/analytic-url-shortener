#[macro_use]
extern crate diesel;

mod handlers;
mod models;
mod schema;

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use dotenv::dotenv;
use r2d2::{self, ConnectionManager};
use std::env;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/endpoints", web::get().to(handlers::get_endpoints))
            .route("/endpoints", web::post().to(handlers::create_endpoint))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
