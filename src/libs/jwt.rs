use actix_web::{web, HttpRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use crate::libs::error;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub iat: usize,
    pub exp: usize,
    pub jid: String,
    pub roles: String,
    pub id: String,
    pub _id: String,
}

pub struct Token {
    pub token: String
}

fn gen_string(size: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}

pub async fn create_jwt(user_id: String, user_type: String, state: &web::Data<AppState>) -> Token {

    let expire = state.config.get::<i64>("jwt.access_expire").unwrap();

    let jwt_key = state.config.get::<String>("jwt.secret_key").unwrap();

    let created = Utc::now();
    let expiry = Utc::now() + Duration::seconds(expire);
    let jid = uuid::Uuid::new_v4();

    let claim = Claims {
        iat: created.timestamp() as usize,
        exp: expiry.timestamp() as usize,
        jid: jid.to_string(),
        roles: user_type,
        id: user_id,
        _id: gen_string(32)
    };

    let token = encode(&Header::new(Algorithm::HS512), &claim, &EncodingKey::from_secret(jwt_key.as_ref())).unwrap();

    Token {
        token
    }
}

pub fn parse_token(token: &str, state: &web::Data<AppState>) -> Result<Claims, error::Error> {

    let jwt_key = state.config.get::<String>("jwt.secret_key").unwrap();

    let token_res = match decode::<Claims>(&token, &DecodingKey::from_secret(jwt_key.as_ref()), &Validation::new(Algorithm::HS512)) {
        Ok(v) => v,
        Err(_) => return Err(error::new_error(1001, "Authentication failure", 401))
    };

    let claims = token_res.claims;

    Ok(claims)
}

pub async fn verify_jwt(req: &HttpRequest, state: &web::Data<AppState>) -> Result<Claims, error::Error> {

    let token = match req.headers().get("Authorization") {
        None => return Err(error::new_error(1001, "Authentication failure", 401)),
        Some(v) => {
            if v.len() <= 6 {
                return Err(error::new_error(1001, "Authentication failure", 401));
            }
            
            let v = v.to_str().unwrap_or_default().to_string();

            if &v[..7] != "Bearer " {
                return Err(error::new_error(1001, "Authentication failure", 401));
            }

            String::from(&v[7..])
        }
    };

    let claims = parse_token(&token, state)?;

    Ok(claims)
}