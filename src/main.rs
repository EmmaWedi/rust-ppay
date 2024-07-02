use std::sync::Arc;

use actix_cors::Cors;
use actix_http::header::{self, HeaderName};
use actix_web::{http, middleware::{ErrorHandlers, Logger}, web::{self, Data}, App, HttpServer, Result};
use setup::databases::mongo::MongoDatabase;
use config::{Config as ConfigLoader, File, FileFormat};
use libs::error;

mod setup;
mod libs;
mod app;

#[derive(Clone)]
pub struct AppState {
    pub config: config::Config,
    pub mongo_db: Arc<Data<MongoDatabase>>,
}

fn load_config() -> Result<config::Config, config::ConfigError> {
    let config = ConfigLoader::builder()
        .set_default("default", "1")?
        .add_source(File::new("src/config/config.app", FileFormat::Toml))
        .build()?;

    Ok(config)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    env_logger::init();

    let settings = load_config().expect("Failed to load configuration");

    let port = settings.get::<String>("app.port").unwrap();
    let host = settings.get::<String>("app.host").unwrap();

    let db_conn = Arc::new(Data::new(MongoDatabase::init(&settings).await));

    let _port = port.clone();
    let _host = host.clone();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
                HeaderName::from_static("x-requested-with"),
            ])
            .supports_credentials()
            .max_age(3600);

        log::info!("==> 🚀 listening at {}:{}", host, port);

        App::new()
            .app_data(web::Data::new(AppState {
                config: settings.clone(),
                mongo_db: db_conn.clone(),
            }))
            .wrap(
                ErrorHandlers::new()
                    .handler(http::StatusCode::METHOD_NOT_ALLOWED, error::render_405)
                    .handler(http::StatusCode::NOT_FOUND, error::render_404)
                    .handler(http::StatusCode::INTERNAL_SERVER_ERROR, error::render_500)
                    .handler(http::StatusCode::BAD_REQUEST, error::render_400),
            )
            .wrap(Logger::default())
            .wrap(cors)
            .configure(app::customers::routes::route::route)
            .configure(app::health::routes::route::route)
    })
    .bind(format!("{}:{}", _host, _port))?
    .run()
    .await
}

