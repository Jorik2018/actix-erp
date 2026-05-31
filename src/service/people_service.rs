use crate::repository::people_repository::PeopleRepository;
use crate::model::people_model::Person;
use chrono::Utc;

pub struct PeopleService {
    repo: PeopleRepository,
}

impl PeopleService {
    pub fn new(repo: PeopleRepository) -> Self {
        Self { repo }
    }

    pub async fn create(&self, mut person: Person) -> surrealdb::Result<Person> {
        let now = Utc::now();
        person.time.created_at = now;
        person.time.updated_at = now;

        self.repo.create(person).await
    }

    pub async fn get_all(&self) -> surrealdb::Result<Vec<Person>> {
        self.repo.find_all().await
    }

    pub async fn get_by_id(&self, id: &str) -> surrealdb::Result<Option<Person>> {
        self.repo.find_by_id(id).await
    }

    pub async fn update(&self, id: &str, mut person: Person) -> surrealdb::Result<Option<Person>> {
        person.time.updated_at = Utc::now();
        self.repo.update(id, person).await
    }

    pub async fn delete(&self, id: &str) -> surrealdb::Result<Option<Person>> {
        self.repo.delete(id).await
    }
}