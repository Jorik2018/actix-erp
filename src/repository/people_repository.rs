use actix_web::web;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use crate::repository::db::get_db;
use crate::model::people_model::Person;
use crate::AppState;

pub struct PeopleRepository {
    state: web::Data<AppState>,
}

impl PeopleRepository {
    pub fn new(state: web::Data<AppState>) -> Self {
        Self { state }
    }

    async fn db(&self) -> surrealdb::Result<Surreal<Any>> {
        get_db(&self.state).await
    }

    pub async fn create(&self, person: Person) -> surrealdb::Result<Person> {
        let db = self.db().await?;

        let created: Option<Person> = db
            .create("person")
            .content(person)
            .await?;
        Ok(created.unwrap())
        //created.ok_or_else(|| surrealdb::Error::Db("No se pudo crear la persona".into()))
    }

    pub async fn find_all(&self) -> surrealdb::Result<Vec<Person>> {
        let db = self.db().await?;
        db.select("person").await
    }

    pub async fn find_by_id(&self, id: &str) -> surrealdb::Result<Option<Person>> {
        let db = self.db().await?;
        db.select(("person", id)).await
    }

    pub async fn update(&self, id: &str, person: Person) -> surrealdb::Result<Option<Person>> {
        let db = self.db().await?;
        db.update(("person", id)).content(person).await
    }

    pub async fn delete(&self, id: &str) -> surrealdb::Result<Option<Person>> {
        let db = self.db().await?;
        db.delete(("person", id)).await
    }
}
