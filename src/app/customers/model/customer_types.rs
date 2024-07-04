use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCustomerModel {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: String,
    pub password: String,
    pub salt: String,
    pub session: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCustomerRequestModel {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: String,
    pub password: String,
}

#[derive(Serialize)]
pub  struct AddCustomerResponseModel<T> {
    pub id: String,
    pub token: String,
    pub code: u16,
    pub status: bool,
    pub message: String,
    pub data: Vec<T>
}

pub enum dataOut<T> {
    Array(Vec<T>),
    Object(HashMap<String, T>),
}

#[derive(Serialize)]
pub struct HttpClientErrorResponse {
    pub code: u16,
    pub status: bool,
    pub message: String,
    pub data: Vec<()>
}

pub struct HttpClientSuccessResponse<T> {
    pub code: u16,
    pub status: bool,
    pub message: String,
    pub data: dataOut<T>
}

pub struct SiginCustomerModel {
    pub email: String,
    pub password: String
}