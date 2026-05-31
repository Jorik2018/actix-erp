use actix_web::web;

use crate::controller::people_controller::{
    create_person,
    get_people,
    get_person,
    update_person,
    delete_person,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/people")
                .route("", web::post().to(create_person))
                .route("", web::get().to(get_people))
                .route("/{id}", web::get().to(get_person))
                .route("/{id}", web::put().to(update_person))
                .route("/{id}", web::delete().to(delete_person))
        );
}