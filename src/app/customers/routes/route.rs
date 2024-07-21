use actix_web::web;

use crate::{
    app::customers::controller::controller::{
        block_customer_controller, create_customer, signin_customer,
    },
    middlewares::{auth::JwtAuthMiddleware, check_user::CheckUserMiddleware},
    AppState,
};

pub fn route(cfg: &mut web::ServiceConfig, state: web::Data<AppState>) {
    cfg.service(
        web::scope("/api/v1/customers")
            .route("/register", web::post().to(create_customer))
            .route("/login", web::post().to(signin_customer))
            .route(
                "/block/{id}", 
                web::get().to(block_customer_controller)
                .wrap(CheckUserMiddleware::new(state.clone()))
                .wrap(JwtAuthMiddleware::new(state.clone()))
            ),
    );
}
