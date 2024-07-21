use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, web, Error, HttpMessage};
use futures::future::{ok, LocalBoxFuture, Ready};
use std::{rc::Rc, sync::Arc};
use std::task::{Context, Poll};

use crate::{app::customers::dto::dto::get_customer, libs::jwt::Claims, AppState};

pub struct CheckUserMiddleware {
    state: web::Data<AppState>,
}

impl CheckUserMiddleware {
    pub fn new(state: web::Data<AppState>) -> Self {
        Self { state }
    }
}

impl<S, B> Transform<S, ServiceRequest> for CheckUserMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CheckUserMiddlewareInner<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckUserMiddlewareInner {
            service: Rc::new(service),
            state: self.state.clone(),
        })
    }
}

pub struct CheckUserMiddlewareInner<S> {
    service: Rc<S>,
    state: web::Data<AppState>,
}

impl<S, B> Service<ServiceRequest> for CheckUserMiddlewareInner<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut user_id: String = "".to_string();

        if let Some(claims) = req.extensions().get::<Arc<Claims>>() {
            user_id = claims.id.clone();
        }

        let service = self.service.clone();
        let state = self.state.clone();
        log::info!("2");
        let fut = async move {
            match get_customer(&state, user_id.as_str()).await {
                Ok(customer) => {
                    req.extensions_mut().insert(customer);
                    service.call(req).await
                }
                Err(_e) => Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
            }
        };
        Box::pin(fut)
    }
}
