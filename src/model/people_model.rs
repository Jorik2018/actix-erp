use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use surrealdb::types::RecordId;
use surrealdb::types::SurrealValue;

#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinates {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub address_line_1: String,
    pub city: String,
    pub coordinates: Coordinates,
    pub country: String,
    pub post_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeInfo {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(SurrealValue, Serialize, Deserialize, Debug)]
pub struct Person {
    pub id: Option<RecordId>,
    pub first_name: String,
    pub last_name: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: Address,
    pub time: TimeInfo,
}

#[derive(SurrealValue,Deserialize)]
pub struct PersonData {
    pub first_name: String,
    pub last_name: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: Address,
    pub time: TimeInfo,
}