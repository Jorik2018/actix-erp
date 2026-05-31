use actix_web::web;
use surrealdb::engine::any;
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::AppState;

pub async fn get_db(state: &web::Data<AppState>) -> surrealdb::Result<Surreal<Any>> {
    let mut guard = state.conn.lock().await;
    let masked_pass = format!("pass=<{}>", "*".repeat(state.config.pass.len()));

    if let Some(db) = guard.as_ref() {
        println!(
            "[get_db] Reutilizando conexión existente | url={} user={} {} ns={} db={}",
            state.config.url,
            state.config.user,
            masked_pass,
            state.config.ns,
            state.config.db
        );
        return Ok(db.clone());
    }

    println!(
        "[get_db] Creando nueva conexión | url={} user={} {} ns={} db={}",
        state.config.url,
        state.config.user,
        masked_pass,
        state.config.ns,
        state.config.db
    );

    let db = match any::connect(&state.config.url).await {
        Ok(db) => db,
        Err(err) => {
            eprintln!("[get_db] Error en connect: {:?}", err);
            panic!("[get_db] Falló any::connect: {:?}", err);
        }
    };

    if let Err(err) = db.signin(Root {
        username: state.config.user.clone(),
        password: state.config.pass.clone(),
    }).await {
        eprintln!("[get_db] Error en signin: {:?}", err);
        panic!("[get_db] Falló signin: {:?}", err);
    }

    if let Err(err) = db.use_ns(&state.config.ns)
        .use_db(&state.config.db)
        .await {
        eprintln!("[get_db] Error en use_ns/use_db: {:?}", err);
        panic!("[get_db] Falló use_ns/use_db: {:?}", err);
    }

    *guard = Some(db.clone());

    println!("[get_db] Conexión establecida correctamente");

    Ok(db)
}
