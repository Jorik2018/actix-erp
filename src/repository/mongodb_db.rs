use actix_web::web;
use mongodb::{Client, Database};

use crate::AppState;

pub async fn get_db(
    state: &web::Data<AppState>,
) -> Result<Database, mongodb::error::Error> {

    let mut guard = state.mongodb_config.conn.lock().await;

    if let Some(db) = guard.as_ref() {
        return Ok(db.clone());
    }

    let client = Client::with_uri_str(&state.mongodb_config.uri).await?;

    let db = client.database(&state.mongodb_config.db);

    *guard = Some(db.clone());

    println!("[get_db] Conexión establecida correctamente");

    Ok(db)
}