#[macro_use]
extern crate diesel;

mod handlers;
mod models;
mod schema;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use diesel::{prelude::*, r2d2};
use dotenv::dotenv;
use std::env;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .route("/r/{endpoint_name}", web::get().to(handlers::index))
            .route("/endpoint", web::get().to(handlers::get_endpoints))
            .route("/endpoint", web::post().to(handlers::create_endpoint))
            .route("/hit/{endpoint_id}", web::get().to(handlers::get_hits))
            .route("/health", web::to(HttpResponse::Ok))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
