use actix_web::web;

use crate::app::customers::controller::controller::{create_customer, signin_customer};

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/customers")
            .route("/register", web::post().to(create_customer))
            .route("/login", web::post().to(signin_customer))
    );
}
