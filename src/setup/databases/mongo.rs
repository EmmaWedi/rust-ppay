use mongodb::{Client, Collection};

use crate::app::customers::model::customer::CustomerModel;

#[derive(Clone, Debug)]
pub struct MongoDatabase {
    pub customers: Collection<CustomerModel>
}

impl MongoDatabase {
    pub async fn init(settings: &config::Config) -> Self {

        let user = settings.get::<String>("mongo.MONGO_DB_USERNAME").unwrap();
        let password = settings.get::<String>("mongo.MONGO_DB_PASSWORD").unwrap();
        let environment = settings.get::<String>("env.environment").unwrap();
        let port = settings.get::<String>("mongo.PORT").unwrap();
        let host_local = settings.get::<String>("mongo.DB_HOST_LOCAL").unwrap();
        let host_prod = settings.get::<String>("mongo.DB_HOST").unwrap();
        let db_name = settings.get::<String>("mongo.DB_NAME").unwrap();
        let url: String;

        match environment.as_str() {
           "TEST" => {
                url = format!("mongodb://{}:{}@{}:{}/{}?directConnection=true", user, password, host_local, port, db_name);
            }
            _ => {
                url = format!("mongodb://{}:{}@{}:{}/{}?directConnection=true", user, password, host_prod, port, db_name);
            }
        }

        let client = Client::with_uri_str(&url).await.unwrap();
        let db = client.database(db_name.as_str());

        let customers: Collection<CustomerModel> = db.collection("customers");

        MongoDatabase {
            customers
        }
    }
}