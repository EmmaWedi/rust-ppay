use actix_web::web;

use crate::app::customers::controller::controller::create_customer;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
        .service(
            web::scope("/customers")
            .route("/register", web::post().to(create_customer))
        )
    );
}