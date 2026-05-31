use crate::model::people_model::Person;
use surrealdb::Surreal;
use surrealdb::engine::any::Any;

pub struct PeopleRepository {
    pub db: Surreal<Any>,
}

impl PeopleRepository {
    pub fn new(db: Surreal<Any>) -> Self {
        Self { db }
    }

    // CREATE
    pub async fn create(&self, person: Person) -> surrealdb::Result<Person> {
        let created: Option<Person> = self
            .db
            .create("person")
            .content(person)
            .await?;

        Ok(created.unwrap())
    }

    // READ ALL
    pub async fn find_all(&self) -> surrealdb::Result<Vec<Person>> {
        let people: Vec<Person> = self
            .db
            .select("person")
            .await?;

        Ok(people)
    }

    // READ ONE
    pub async fn find_by_id(&self, id: &str) -> surrealdb::Result<Option<Person>> {
        let person: Option<Person> = self
            .db
            .select(("person", id))
            .await?;

        Ok(person)
    }

    // UPDATE
    pub async fn update(&self, id: &str, person: Person) -> surrealdb::Result<Option<Person>> {
        let updated: Option<Person> = self
            .db
            .update(("person", id))
            .content(person)
            .await?;

        Ok(updated)
    }

    // DELETE
    pub async fn delete(&self, id: &str) -> surrealdb::Result<Option<Person>> {
        let deleted: Option<Person> = self
            .db
            .delete(("person", id))
            .await?;

        Ok(deleted)
    }
}