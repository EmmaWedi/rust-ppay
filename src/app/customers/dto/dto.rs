use actix_web::web;
use futures_util::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    results::{InsertOneResult, UpdateResult},
};

use crate::{
    app::customers::model::{
        customer::CustomerModel,
        customer_types::{AddCustomerModel, Customer},
    },
    AppState,
};

pub async fn create_customer_dto(
    state: &web::Data<AppState>,
    add_customer_model: AddCustomerModel,
) -> Result<InsertOneResult, Error> {
    let fullname = format!(
        "{} {}",
        add_customer_model.firstname.clone(),
        add_customer_model.lastname.clone()
    );

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
        session_id: add_customer_model.session,
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

pub async fn get_customer(state: &web::Data<AppState>, id: &str) -> Result<Customer, Error> {
    let result = state
        .mongo_db
        .customers
        .find_one(doc! { "session_id": id, "is_blocked": false })
        .await
        .ok()
        .expect("Could Not Get Customer");

    match result {
        Some(doc) => {
            let customer = doc.into();
            Ok(customer)
        }
        None => Err(Error::custom("Could Not Get Customer".to_string())),
    }
}

pub async fn get_all_customers(state: &web::Data<AppState>) -> Result<Vec<Customer>, Error> {
    let mut results = state
        .mongo_db
        .customers
        .find(doc! {})
        .await?;
    
    let mut customers: Vec<Customer> = Vec::new();

    while let Some(result) = results.next().await {
        match result {
            Ok(doc) => {
                customers.push(doc.into());
            }
            Err(_) => {
                Error::custom("Failed To Fetch Customers".to_string());
            }
        }
    }

    Ok(customers)
}

pub async fn get_customer_by_email(
    state: &web::Data<AppState>,
    email: &str,
) -> Result<Customer, Error> {
    let result = state
        .mongo_db
        .customers
        .find_one(doc! { "email": email, "is_blocked": false })
        .await?;

    match result {
        Some(doc) => {
            let customer = doc.into();
            Ok(customer)
        }
        None => Err(Error::custom("Could Not Get Customer".to_string())),
    }
}

pub async fn block_customer(state: &web::Data<AppState>, id: &str) -> Result<UpdateResult, Error> {
    let result = state
        .mongo_db
        .customers
        .update_one(
            doc! {
                "session_id": id
            },
            doc! {
                "$set": doc! {
                    "is_blocked": true
                }
            },
        )
        .await
        .ok()
        .expect("Could Not Block Customer");

    Ok(result)
}

pub async fn update_session(state: &web::Data<AppState>, session: String, id: ObjectId) -> Result<UpdateResult, Error> {
    let result = state
        .mongo_db
        .customers
        .update_one(
            doc! {
                "_id": id
            },
            doc! {
                "$set": doc! {
                    "session_id": session
                }
            }
        )
        .await
        .ok()
        .expect("Could Not Update Session");

    Ok(result)
}