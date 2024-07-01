use std::fmt::{Display, Formatter, Result as FmtResult};
use actix_http::StatusCode;
use actix_web::{dev, http, middleware::ErrorHandlerResponse, HttpResponse, ResponseError, Result};
use serde_json::{json, to_string_pretty};
use serde::Serialize;
use actix_http::body::{EitherBody, BoxBody};

#[derive(Debug, Serialize)]
pub struct Error {
    pub error_msg: String,
    pub error_code: u32,
    pub error_status: u16,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let err_json = json!({ "error_code": self.error_code, "error_msg": self.error_msg });
        HttpResponse::build(StatusCode::from_u16(self.error_status).unwrap()).json(err_json)
    }
}

pub fn new_error(error_code: u32, error_msg: &str, error_status: u16) -> Error {
    Error {
        error_msg: error_msg.to_string(),
        error_code,
        error_status,
    }
}

// pub fn res_error(error_code: u32, error_msg: &str, error_status: u16) -> Result<HttpResponse, Error> {
//     Err(Error {
//         error_msg: error_msg.to_string(),
//         error_code,
//         error_status,
//     })
// }

// pub fn error_500() -> Error {
//     Error {
//         error_msg: "Internal Server Error".to_string(),
//         error_code: 500,
//         error_status: 500,
//     }
// }

pub fn render_404<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::header::HeaderValue::from_static("application/json"),
    );

    let new_res = res.map_body(|_, _| {
        EitherBody::left(BoxBody::new("{\"error_code\": 404, \"error_msg\": \"Not Found\"}"))
    });
    Ok(ErrorHandlerResponse::Response(new_res))
}

pub fn render_405<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::header::HeaderValue::from_static("application/json"),
    );

    let new_res = res.map_body(|_, _| {
        EitherBody::left(BoxBody::new("{\"error_code\": 405, \"error_msg\": \"Method Not Allowed\"}"))
    });
    Ok(ErrorHandlerResponse::Response(new_res))
}

pub fn render_500<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::header::HeaderValue::from_static("application/json"),
    );

    let new_res = res.map_body(|_, _| {
        EitherBody::left(BoxBody::new("{\"error_code\": 500, \"error_msg\": \"Internal Server Error\"}"))
    });
    Ok(ErrorHandlerResponse::Response(new_res))
}

pub fn render_400<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::header::HeaderValue::from_static("application/json"),
    );

    let new_res = res.map_body(|_, _| {
        EitherBody::left(BoxBody::new("{\"error_code\": 400, \"error_msg\": \"Bad Request\"}"))
    });
    Ok(ErrorHandlerResponse::Response(new_res))
}
