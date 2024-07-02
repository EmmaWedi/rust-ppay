use actix_web::web;

use crate::app::health::controller::controller::engine_check;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
        .service(
            web::scope("/health")
            .route("/engine", web::get().to(engine_check))
        )
    );
}