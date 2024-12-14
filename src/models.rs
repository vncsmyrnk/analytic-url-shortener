use crate::schema::{endpoints, hits};
use chrono::NaiveDateTime;
use ipnetwork::IpNetwork;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Endpoint {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = endpoints)]
pub struct NewEndpoint {
    pub name: String,
    pub url: String,
}

#[derive(Queryable, Serialize)]
pub struct Hit {
    pub id: i32,
    pub endpoint_id: i32,
    pub hit_time: Option<NaiveDateTime>,
    pub ip: IpNetwork,
    pub user_agent: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = hits)]
pub struct NewHit {
    pub endpoint_id: i32,
    pub hit_time: Option<NaiveDateTime>,
    pub ip: IpNetwork,
    pub user_agent: Option<String>,
}
