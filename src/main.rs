use std::env;
use actix_web::{App, HttpServer, web};
use tokio::sync::Mutex;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use mongodb::Database;

mod routes;
mod model;
mod repository;
mod service;
mod controller;
mod commons;
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
//#[derive(Clone)]
pub struct MongoDBConfig {
    pub uri: String,
    pub db: String,
    pub conn: Mutex<Option<Database>>,
}

pub struct AppState {
    pub config: DbConfig,
    pub mongodb_config: MongoDBConfig,
    pub conn: Mutex<Option<Surreal<Any>>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    unsafe { std::env::set_var("RUST_LOG", "actix_web=debug") };
    
    let config = DbConfig {
        url: env::var("SURREALDB_URL").unwrap_or_default(),
        user: env::var("SURREALDB_USER").unwrap_or_default(),
        pass: env::var("SURREALDB_PASS").unwrap_or_default(),
        ns: env::var("SURREALDB_NS").unwrap_or_else(|_| "demo".into()),
        db: env::var("SURREALDB_DB").unwrap_or_else(|_| "surreal_deal_store".into()),
    };

    let mongodb_config = MongoDBConfig {
        uri: env::var("MONGO_URI").unwrap_or_default(),
        db: env::var("DATABASE_NAME").unwrap_or_else(|_| "db".into()),
        conn: Mutex::new(None),
    };

    let state = web::Data::new(AppState {
        config,
        mongodb_config,
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
