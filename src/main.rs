use std::env;
use actix_web::{App, HttpServer, web};
use surrealdb::engine::any;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

mod routes;
mod model;
mod repository;
mod service;
mod controller;

use repository::people_repository::PeopleRepository;
use service::people_service::PeopleService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // 🔌 Conexión a SurrealDB
let db_url = env::var("SURREALDB_URL").expect("SURREALDB_URL no definido");
let db_user = env::var("SURREALDB_USER").expect("SURREALDB_USER no definido");
let db_pass = env::var("SURREALDB_PASS").expect("SURREALDB_PASS no definido");
let db_ns = env::var("SURREALDB_NS").unwrap_or("demo".into());
let db_name = env::var("SURREALDB_DB").unwrap_or("surreal_deal_store".into());

let db = any::connect(db_url)
    .await
    .expect("Error conectando a SurrealDB");

db.signin(Root {
    username: db_user,
    password: db_pass,
}).await.expect("Error en login");

db.use_ns(db_ns)
    .use_db(db_name)
    .await
    .expect("Error seleccionando DB");

    // 🧱 Inyección de dependencias
    let repo = PeopleRepository::new(db);
    let service = web::Data::new(PeopleService::new(repo));

    println!("🚀 Server corriendo en http://127.0.0.1:8080");

    // 🌐 Servidor Actix
    HttpServer::new(move || {
        App::new()
            .app_data(service.clone())
            .configure(routes::people_routes::config) // tu config aquí
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}