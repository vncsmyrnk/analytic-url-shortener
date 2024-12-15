use std::str::FromStr;

use crate::models::{Endpoint, Hit, NewEndpoint, NewHit};
use crate::schema::{endpoints, hits};
use crate::DbPool;
use actix_web::{http::header, web, HttpRequest, HttpResponse, Responder};
use chrono::Utc;
use diesel::prelude::*;
use ipnetwork::IpNetwork;
use log::error;
use uuid::Uuid;

pub async fn index(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let ip_address = req
        .peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "0.0.0.0/24".to_string());

    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|ua| ua.to_str().ok())
        .map(|ua| if ua.len() > 100 { &ua[..100] } else { ua })
        .unwrap_or("Unknown User-Agent")
        .to_string();

    let result = web::block(move || -> Result<String, diesel::result::Error> {
        use crate::schema::endpoints::dsl::*;

        let endpoint = endpoints
            .filter(hash.eq(path.into_inner()))
            .first::<Endpoint>(&mut conn)?;

        let new_hit = NewHit {
            endpoint_id: endpoint.id,
            hit_time: Some(Utc::now().naive_utc()),
            ip: IpNetwork::from_str(&ip_address).unwrap(),
            user_agent: Some(user_agent),
        };

        diesel::insert_into(hits::table)
            .values(&new_hit)
            .get_result::<Hit>(&mut conn)?;

        Ok(endpoint.url)
    })
    .await
    .map_err(|e| {
        error!("{}", e);
        HttpResponse::InternalServerError().finish()
    });

    match result {
        Ok(url) => match url {
            Ok(value) => HttpResponse::Found()
                .append_header((header::LOCATION, value))
                .finish(),
            Err(e) => {
                error!("{}", e);
                HttpResponse::NotFound().finish()
            }
        },
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn get_endpoints(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let result = web::block(move || endpoints::table.load::<Endpoint>(&mut conn))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        });

    match result {
        Ok(endpoints) => HttpResponse::Ok().json(endpoints.unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_hits(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let result = web::block(move || {
        use crate::schema::hits::dsl::*;

        hits.filter(endpoint_id.eq(path.into_inner()))
            .load::<Hit>(&mut conn)
    })
    .await
    .map_err(|e| {
        error!("{}", e);
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
    let mut new_endpoint = item.into_inner();
    let my_uuid = Uuid::new_v4();
    new_endpoint.hash = Some(my_uuid.to_string().chars().take(8).collect::<String>());

    let result = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get db connection from pool");
        diesel::insert_into(endpoints::table)
            .values(&new_endpoint)
            .get_result::<Endpoint>(&mut conn)
    })
    .await
    .map_err(|e| {
        error!("{}", e);
        HttpResponse::InternalServerError().finish()
    });

    match result {
        Ok(endpoint) => HttpResponse::Ok().json(endpoint.unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
