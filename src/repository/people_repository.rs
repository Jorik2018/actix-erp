use crate::AppState;
use crate::commons::{AppError, AppResult};
use crate::model::people_model::Person;
use crate::repository::surrealdb_db::get_db;
use actix_web::web;
use surrealdb::Surreal;
use surrealdb::engine::any::Any;

pub struct PeopleRepository {
    state: web::Data<AppState>,
}

impl PeopleRepository {

    pub fn new(state: web::Data<AppState>) -> Self {
        Self { state }
    }

    async fn db(&self) -> AppResult<Surreal<Any>> {
        get_db(&self.state)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn create(&self, person: Person) -> AppResult<Person> {
        let db = self.db().await?;
        let created: Option<Person> = db
            .create("person")
            .content(person)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        created.ok_or(AppError::Database(
            "No se pudo crear la persona".to_string(),
        ))
    }

    pub async fn find_all(&self) -> AppResult<Vec<Person>> {
        let db = self.db().await?;
        db.select("person")
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn find_by_id(&self, id: &str) -> AppResult<Option<Person>> {
        let db = self.db().await?;
        db.select(("person", id))
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn update(&self, id: &str, person: Person) -> AppResult<Option<Person>> {
        let db = self.db().await?;
        db.update(("person", id))
            .content(person)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(&self, id: &str) -> AppResult<Option<Person>> {
        let db = self.db().await?;
        db.delete(("person", id))
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

}
