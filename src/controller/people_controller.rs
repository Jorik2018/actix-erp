use actix_web::{web, HttpResponse, Responder};
use crate::service::people_service::PeopleService;
use crate::model::people_model::Person;

pub async fn create_person(
    service: web::Data<PeopleService>,
    payload: web::Json<Person>,
) -> impl Responder {
    match service.create(payload.into_inner()).await {
        Ok(person) => HttpResponse::Ok().json(person),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_people(
    service: web::Data<PeopleService>,
) -> impl Responder {
    match service.get_all().await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_person(
    service: web::Data<PeopleService>,
    path: web::Path<String>,
) -> impl Responder {
    match service.get_by_id(&path).await {
        Ok(Some(person)) => HttpResponse::Ok().json(person),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_person(
    service: web::Data<PeopleService>,
    path: web::Path<String>,
    payload: web::Json<Person>,
) -> impl Responder {
    match service.update(&path, payload.into_inner()).await {
        Ok(Some(person)) => HttpResponse::Ok().json(person),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_person(
    service: web::Data<PeopleService>,
    path: web::Path<String>,
) -> impl Responder {
    match service.delete(&path).await {
        Ok(Some(_)) => HttpResponse::Ok().finish(),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}