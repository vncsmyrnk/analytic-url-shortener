use crate::models::{Endpoint, Hit, NewEndpoint, NewHit};
use crate::schema::{endpoints, hits};
use crate::DbPool;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

pub async fn get_endpoints(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    let result = web::block(move || endpoints::table.load::<Endpoint>(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        });

    match result {
        Ok(endpoints) => HttpResponse::Ok().json(endpoints),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_endpoint(
    pool: web::Data<DbPool>,
    item: web::Json<NewEndpoint>,
) -> impl Responder {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    let new_endpoint = item.into_inner();

    let result = web::block(move || {
        diesel::insert_into(endpoints::table)
            .values(&new_endpoint)
            .get_result::<Endpoint>(&conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    });

    match result {
        Ok(endpoint) => HttpResponse::Ok().json(endpoint),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
