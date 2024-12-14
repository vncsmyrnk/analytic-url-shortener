use crate::models::{Endpoint, Hit, NewEndpoint, NewHit};
use crate::schema::{endpoints, hits};
use crate::DbPool;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
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

pub async fn get_hits(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let result = web::block(move || hits::table.load::<Hit>(&mut conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        });

    match result {
        Ok(hits) => HttpResponse::Ok().json(hits.unwrap()),
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

pub async fn create_hit(pool: web::Data<DbPool>, item: web::Json<NewHit>) -> impl Responder {
    let mut new_hit = item.into_inner();
    new_hit.hit_time = Some(Utc::now().naive_utc());

    let result = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get db connection from pool");
        diesel::insert_into(hits::table)
            .values(&new_hit)
            .get_result::<Hit>(&mut conn)
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
