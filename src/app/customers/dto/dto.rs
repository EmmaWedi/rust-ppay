use actix_web::web;
use mongodb::{error::Error, results::InsertOneResult};

use crate::{app::customers::model::customer::{AddCustomerModel, CustomerModel}, AppState};

pub async fn create_customer_dto(state: &web::Data<AppState>, add_customer_model: AddCustomerModel) -> Result<InsertOneResult, Error> {

    let fullname = format!("{} {}", add_customer_model.firstname.clone(), add_customer_model.lastname.clone());

    let new_user = CustomerModel {
        id: None,
        first_name: add_customer_model.firstname,
        last_name: add_customer_model.lastname,
        referal_code: None,
        full_name: fullname,
        email: add_customer_model.email,
        phone: add_customer_model.phone,
        password: add_customer_model.password,
        phone_verified: false,
        email_verified: false,
        is_subscribed_on_bvirtual: false,
        is_account_active: false,
        is_blocked: false,
        created_at: mongodb::bson::DateTime::now(),
        updated_at: mongodb::bson::DateTime::now(),
        last_seen: None,
        profile: "subscriber".to_string(),
        salt: add_customer_model.salt,
        session_id: add_customer_model.session
    };

    let result = state
        .mongo_db
        .customers
        .insert_one(new_user)
        .await
        .ok()
        .expect("Could not insert document");
        
    Ok(result)
}