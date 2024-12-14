use crate::models::{Endpoint, NewEndpoint};
use crate::schema::endpoints;
use crate::DbPool;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

pub async fn get_endpoints(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let result = web::block(move || endpoints::table.load::<Endpoint>(&mut conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        });

    match result {
        Ok(endpoints) => HttpResponse::Ok().json(endpoints.unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_endpoint(
    pool: web::Data<DbPool>,
    item: web::Json<NewEndpoint>,
) -> impl Responder {
    let new_endpoint = item.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get db connection from pool");
        diesel::insert_into(endpoints::table)
            .values(&new_endpoint)
            .get_result::<Endpoint>(&mut conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    });

    match result {
        Ok(endpoint) => HttpResponse::Ok().json(endpoint.unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
