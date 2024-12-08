use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Endpoint {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub method: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "endpoints"]
pub struct NewEndpoint {
    pub name: String,
    pub url: String,
    pub method: String,
}

#[derive(Queryable, Serialize)]
pub struct Hit {
    pub id: i32,
    pub endpoint_id: i32,
    pub hit_time: chrono::NaiveDateTime,
    pub client_ip: std::net::IpAddr,
    pub status_code: i32,
    pub response_time: i32,
    pub country_code: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "hits"]
pub struct NewHit {
    pub endpoint_id: i32,
    pub client_ip: std::net::IpAddr,
    pub status_code: i32,
    pub response_time: i32,
    pub country_code: Option<String>,
}
