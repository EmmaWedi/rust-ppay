use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, web, Error, HttpMessage};
use futures::future::{ok, LocalBoxFuture, Ready};
use std::{rc::Rc, sync::Arc};
use std::task::{Context, Poll};

use crate::{libs::jwt::verify_jwt, AppState};

pub struct JwtAuthMiddleware {
    state: web::Data<AppState>,
}

impl JwtAuthMiddleware {
    pub fn new(state: web::Data<AppState>) -> Self {
        Self { state }
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtAuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtAuthMiddlewareInner<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtAuthMiddlewareInner {
            service: Rc::new(service),
            state: self.state.clone(),
        })
    }
}

pub struct JwtAuthMiddlewareInner<S> {
    service: Rc<S>,
    state: web::Data<AppState>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddlewareInner<S>
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
        let service = self.service.clone();
        let state = self.state.clone();
        let (http_request, payload) = req.into_parts();

        let fut = async move {
            match verify_jwt(&http_request, &state).await {
                Ok(claims) => {
                    let req = ServiceRequest::from_parts(http_request, payload);
                    req.extensions_mut().insert(Arc::new(claims));
                    service.call(req).await
                }
                Err(_) => Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
            }
        };

        Box::pin(fut)
    }
}