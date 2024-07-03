use actix_web::{web, HttpResponse};

use crate::{app::customers::{dto::dto::create_customer_dto, model::customer::{AddCustomerModel, AddCustomerRequestModel, AddCustomerResponseModel, HttpClientErrorResponse, SiginCustomerModel }}, libs::{error, jwt::create_jwt, util::{encrypt_password, salt}, validator}, AppState};

pub async fn create_customer(payload: web::Json<AddCustomerRequestModel>, state: web::Data<AppState>) -> Result<HttpResponse, error::Error> {
    let firstname = validator::required_str(&payload.firstname, "First Name")?;
    let password = validator::required_str(&payload.password, "Password")?;
    let email = validator::email(&payload.email, "Email")?;
    let lastname = validator::required_str(&payload.lastname, "Last Name")?;
    let mobile =  validator::mobile(&payload.phone, "Phone")?;

    let salt = salt();
    let hash_password = encrypt_password(&password, &salt);

    let data = AddCustomerModel {
        firstname,
        lastname,
        email,
        phone: mobile,
        password: hash_password,
        salt: salt.to_string(),
        session: uuid::Uuid::new_v4().to_string()
    };

    let result = create_customer_dto(&state, data).await;

    match result {
        Ok(res) => {
            let _id = res.inserted_id.to_string();
            
            let token_addr = create_jwt(_id.clone(), "user".to_string(), &state).await;

            Ok(HttpResponse::Ok().json(AddCustomerResponseModel {
                id: _id,
                token: token_addr.token,
                code: 2000,
                status: true,
                message: "Success".to_string(),
                data: vec![res]
            }))
        }
        Err(e) => {
            Ok(HttpResponse::BadRequest().json(HttpClientErrorResponse {
                code: 2002,
                status: false,
                message: e.to_string(),
                data: vec![],
            }))
        }
    }
}

pub async fn signin_customer() -> Result<HttpResponse, error::Error> {
    Ok(HttpResponse::Ok().body("Working"))
}