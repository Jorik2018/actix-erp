use std::env;
use actix_web::{App, HttpServer, web};
use tokio::sync::Mutex;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;

mod routes;
mod model;
mod repository;
mod service;
mod controller;

use repository::people_repository::PeopleRepository;
use service::people_service::PeopleService;

#[derive(Clone)]
pub struct DbConfig {
    pub url: String,
    pub user: String,
    pub pass: String,
    pub ns: String,
    pub db: String,
}

pub struct AppState {
    pub config: DbConfig,
    pub conn: Mutex<Option<Surreal<Any>>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = DbConfig {
        url: env::var("SURREALDB_URL").unwrap_or_default(),
        user: env::var("SURREALDB_USER").unwrap_or_default(),
        pass: env::var("SURREALDB_PASS").unwrap_or_default(),
        ns: env::var("SURREALDB_NS").unwrap_or_else(|_| "demo".into()),
        db: env::var("SURREALDB_DB").unwrap_or_else(|_| "surreal_deal_store".into()),
    };

    let state = web::Data::new(AppState {
        config,
        conn: Mutex::new(None),
    });

    let repo = PeopleRepository::new(state.clone());
    let service = web::Data::new(PeopleService::new(repo));

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(service.clone())
            .configure(routes::people_routes::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
