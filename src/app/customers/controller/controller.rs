use std::collections::HashMap;

use actix_web::{web, HttpResponse};

use crate::{
    app::customers::{
        dto::dto::{block_customer, create_customer_dto, get_customer_by_email, update_session},
        model::customer_types::{
            AddCustomerModel, AddCustomerRequestModel, AddCustomerResponseModel, CustomerToReturnModel, DataOut, HttpClientErrorResponse, HttpClientSuccessResponse, IdPathModel, SiginCustomerRequestModel
        },
    },
    libs::{
        error,
        jwt::create_jwt,
        util::{encrypt_password, parse_uuid, salt, validate_password},
        validator,
    },
    AppState,
};

pub async fn create_customer(
    payload: web::Json<AddCustomerRequestModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let firstname = validator::required_str(&payload.firstname, "First Name")?;
    let password = validator::required_str(&payload.password, "Password")?;
    let email = validator::email(&payload.email, "Email")?;
    let lastname = validator::required_str(&payload.lastname, "Last Name")?;
    let mobile = validator::mobile(&payload.phone, "Phone")?;

    if let Ok(customer) = get_customer_by_email(&state, &email).await {
        return Ok(HttpResponse::Ok().json(HttpClientErrorResponse {
            code: 2002,
            status: false,
            message: format!("Customer With Email {} Exists", customer.email),
            data: vec![],
        }));
    }

    let salt = salt();
    let hash_password = encrypt_password(&password, &salt);
    let session = uuid::Uuid::new_v4().to_string();

    let data = AddCustomerModel {
        firstname,
        lastname,
        email,
        phone: mobile,
        password: hash_password,
        salt: salt.to_string(),
        session: session.clone(),
    };

    let result = create_customer_dto(&state, data).await;

    match result {
        Ok(res) => {

            let token_addr = create_jwt(session.clone(), "user".to_string(), &state).await;

            let mut map_res = HashMap::new();
            map_res.insert("matched".to_string(), res);

            Ok(HttpResponse::Ok().json(AddCustomerResponseModel {
                id: session,
                token: token_addr.token,
                code: 2000,
                status: true,
                message: "Success".to_string(),
                data: DataOut::Result(map_res),
            }))
        }
        Err(_) => Ok(HttpResponse::Ok().json(HttpClientErrorResponse {
            code: 2002,
            status: false,
            message: "Could not register".to_string(),
            data: vec![],
        })),
    }
}

pub async fn signin_customer(
    payload: web::Json<SiginCustomerRequestModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let password = validator::required_str(&payload.password, "Password")?;
    let email = validator::email(&payload.email, "Email")?;

    match get_customer_by_email(&state, &email).await {
        Ok(customer) => {
            let salt = parse_uuid(&customer.salt);

            let session = uuid::Uuid::new_v4().to_string();

            let valid_password = validate_password(&password, &salt, &customer.password);

            if !valid_password {
                return Ok(HttpResponse::Ok().json(HttpClientErrorResponse {
                    code: 2002,
                    status: false,
                    message: "Invalid Credentials".to_string(),
                    data: vec![],
                }));
            }

            let _ = update_session(&state, session.clone(), customer._id).await;

            let token_addr = create_jwt(session.clone(), "user".to_string(), &state).await;

            let cus = CustomerToReturnModel {
                first_name: customer.first_name,
                last_name: customer.last_name,
                email: customer.email,
                phone: customer.phone,
                referal_code: customer.referal_code,
                profile: customer.profile,
                email_verified: customer.email_verified,
                phone_verified: customer.phone_verified,
                is_subscribed_on_bvirtual: customer.is_subscribed_on_bvirtual,
                is_account_active: customer.is_account_active,
                last_seen: customer.last_seen.to_string()
            };

            let mut map_res = HashMap::new();
            map_res.insert("customer".to_string(), cus);

            Ok(HttpResponse::Ok().json(AddCustomerResponseModel {
                id: session,
                token: token_addr.token,
                code: 2000,
                status: true,
                message: "Success".to_string(),
                data: DataOut::Result(map_res),
            }))
        }
        Err(_) => Ok(HttpResponse::Ok().json(HttpClientErrorResponse {
            code: 2002,
            status: false,
            message: "Wrong Credentials".to_string(),
            data: vec![],
        })),
    }
}

pub async fn block_customer_controller(params: web::Path<IdPathModel>, state: web::Data<AppState>) -> Result<HttpResponse, error::Error> {
    let id = validator::uuid(&params.id, "Id")?;
    
    let result = block_customer(&state, &id).await;

    match result {
        Ok(res) => {

            let mut map_res = HashMap::new();
            map_res.insert("matched".to_string(), res.matched_count);
            map_res.insert("modified".to_string(), res.modified_count);

            Ok(HttpResponse::Ok().json(HttpClientSuccessResponse {
                code: 2000,
                status: true,
                message: "Success".to_string(),
                data: DataOut::Result(map_res)
            }))        
        }
        Err(_) => Ok(HttpResponse::Ok().json(HttpClientErrorResponse {
            code: 2002,
            status: false,
            message: "Wrong Credentials".to_string(),
            data: vec![],
        }))
    }
}
