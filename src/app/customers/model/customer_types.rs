use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::app::customers::model::customer::CustomerModel;

//Request Models
#[derive(Debug, Serialize, Deserialize)]
pub struct AddCustomerRequestModel {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiginCustomerRequestModel {
    pub email: String,
    pub password: String
}

#[derive(Deserialize, Serialize)]
pub struct IdPathModel {
    pub id: String
}

//Response Models
#[derive(Serialize)]
pub  struct AddCustomerResponseModel {
    pub id: String,
    pub token: String,
    pub code: u16,
    pub status: bool,
    pub message: String,
    pub data: Value
}

#[derive(Serialize)]
pub struct HttpClientResponse {
    pub code: u16,
    pub status: bool,
    pub message: String,
    pub data: Value
}

//Database Models
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
pub struct Customer {
    pub _id: ObjectId,
    pub first_name: String,
    pub full_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub session_id: String,
    pub referal_code: String,
    pub profile: String,
    pub phone_verified: bool,
    pub email_verified: bool,
    pub is_subscribed_on_bvirtual: bool,
    pub is_account_active: bool,
    pub is_blocked: bool,
    pub last_seen: DateTime,
    pub salt: String,
    pub password: String
}

impl From<CustomerModel> for Customer {
    fn from(model: CustomerModel) -> Self {
        Customer {
            _id: model.id.unwrap_or_else(ObjectId::new),
            first_name: model.first_name,
            full_name: model.full_name,
            last_name: model.last_name,
            email: model.email,
            phone: model.phone,
            session_id: model.session_id,
            referal_code: model.referal_code.unwrap_or_else(|| "".to_string()),
            profile: model.profile,
            phone_verified: model.phone_verified,
            email_verified: model.email_verified,
            is_subscribed_on_bvirtual: model.is_subscribed_on_bvirtual,
            is_account_active: model.is_account_active,
            is_blocked: model.is_blocked,
            last_seen: model.last_seen.unwrap_or_else(DateTime::now),
            salt: model.salt,
            password: model.password
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerToReturnModel {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub referal_code: String,
    pub profile: String,
    pub phone_verified: bool,
    pub email_verified: bool,
    pub is_subscribed_on_bvirtual: bool,
    pub is_account_active: bool,
    pub last_seen: String,
}