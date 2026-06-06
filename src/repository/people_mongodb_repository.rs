use actix_web::web;
use futures::stream::TryStreamExt;
use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};
use crate::{
    AppState,
    commons::{AppError, AppResult},
    model::people_model::Person,
    repository::mongodb_db::get_db,
};

pub struct PeopleRepository {
    state: web::Data<AppState>,
}

impl PeopleRepository {
    pub fn new(state: web::Data<AppState>) -> Self {
        Self { state }
    }

    async fn collection(&self) -> AppResult<Collection<Person>> {
        let db = get_db(&self.state)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(db.collection::<Person>("person"))
    }

    fn object_id(&self, id: &str) -> AppResult<ObjectId> {
        ObjectId::parse_str(id).map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn create(&self, mut person: Person) -> AppResult<Person> {
        let result = self
            .collection()
            .await?
            .insert_one(&person)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        if let Some(id) = result.inserted_id.as_object_id() {
            //person.id = Some(id.to_hex());
        }

        Ok(person)
    }

    pub async fn find_all(&self) -> AppResult<Vec<Person>> {
        let cursor = self.collection().await?
            .find(doc! {})
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        cursor
            .try_collect::<Vec<Person>>()
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn find_by_id(&self, id: &str) -> AppResult<Option<Person>> {
        let object_id = self.object_id(id)?;
        self.collection().await?
            .find_one(doc! { "_id": object_id })
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn update(&self, id: &str, person: Person) -> AppResult<Option<Person>> {
        let collection = self.collection().await?;
        let object_id = self.object_id(id)?;

        collection
            .replace_one(doc! { "_id": object_id }, person)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        collection
            .find_one(doc! { "_id": object_id })
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(&self, id: &str) -> AppResult<Option<Person>> {
        let collection = self.collection().await?;
        let object_id = self.object_id(id)?;

        let person = collection
            .find_one(doc! { "_id": object_id })
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        if person.is_some() {
            collection
                .delete_one(doc! { "_id": object_id })
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
        }

        Ok(person)
    }
}
