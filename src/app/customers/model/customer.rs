use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCustomerModel {
    pub first_name: String,
    pub last_name: String
}

impl TryFrom<AddCustomerModel> for CustomerModel {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: AddCustomerModel) -> Result<Self, Self::Error> {
        
        Ok(Self {
            id: None,
            first_name: item.first_name,
            last_name: item.last_name
        })
    }
}