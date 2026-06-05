use actix_web::{web, HttpResponse, Responder};
use crate::service::people_service::PeopleService;
use crate::model::people_model::Person;

pub async fn greeting(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("hi {}", name.into_inner()))
}

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
        Err(e) => {
            eprintln!("Error en get_people: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("{:?}", e)
            }))
        }
    }
}

pub async fn get_person(
    service: web::Data<PeopleService>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    match service.get_by_id(&id).await {
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
    let id = path.into_inner();
    match service.update(&id, payload.into_inner()).await {
        Ok(Some(person)) => HttpResponse::Ok().json(person),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_person(
    service: web::Data<PeopleService>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    match service.delete(&id).await {
        Ok(Some(_)) => HttpResponse::Ok().finish(),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

