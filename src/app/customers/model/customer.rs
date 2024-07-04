use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    
    #[serde(default = "default_profile")]
    pub profile: String,
    
    pub referal_code: Option<String>,
    pub full_name: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub password: String,
    pub salt: String,
    pub session_id: String,
    
    #[serde(default = "default_false")]
    pub phone_verified: bool,

    #[serde(default = "default_false")]
    pub email_verified: bool,

    pub is_subscribed_on_bvirtual: bool,
    
    #[serde(default = "default_false")]
    pub is_account_active: bool,
    
    #[serde(default = "default_false")]
    pub is_blocked: bool,
    
    #[serde(default = "now")]
    pub created_at: DateTime,
    
    #[serde(default = "now")]
    pub updated_at: DateTime,
    
    #[serde(default)]
    pub last_seen: Option<DateTime>,
}

fn default_profile() -> String {
    "subscriber".to_string()
}

fn default_false() -> bool {
    false
}

fn now() -> DateTime {
    DateTime::now()
}