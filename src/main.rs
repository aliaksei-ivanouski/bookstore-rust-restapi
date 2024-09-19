#[macro_use]
extern crate rocket;

use rocket::http::Status;
use sea_orm_migration::MigratorTrait;
use controllers::{Response, SuccessResponse};
use fairings::cors::{options, CORS};
use migrator::Migrator;
use rocket_prometheus::{PrometheusMetrics};

mod controllers;
mod db;
mod entities;
mod fairings;
mod migrator;
mod auth;
mod metrics;

pub struct AppConfig {
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_database: String,
    jwt_secret: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_host: std::env::var("DB_HOST").unwrap_or("localhost".to_string()),
            db_port: std::env::var("DB_PORT").unwrap_or("13306".to_string()),
            db_username: std::env::var("DB_USERNAME").unwrap_or("root".to_string()),
            db_password: std::env::var("DB_PASSWORD").unwrap_or("12345678".to_string()),
            db_database: std::env::var("DB_DATABASE").unwrap_or("bookstore".to_string()),
            jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
        }
    }
}

#[get("/")]
fn index() -> Response<String> {
    metrics::metrics::HTTP_REQUESTS_TOTAL.with_label_values(&["index"]).inc();
    Ok(SuccessResponse((Status::Ok, "Hello, world!".to_string())))
}

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().ok();
    let config = AppConfig::default();
    let db = db::connect(&config).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    let prometheus = PrometheusMetrics::new();
    prometheus
        .registry()
        .register(Box::new(metrics::metrics::HTTP_REQUESTS_TOTAL.clone()))
        .unwrap();
    prometheus
        .registry()
        .register(Box::new(metrics::metrics::HTTP_CONNECTED_SSE_CLIENTS.clone()))
        .unwrap();
    prometheus
        .registry()
        .register(Box::new(metrics::metrics::HTTP_RESPONSE_TIME_SECONDS.clone()))
        .unwrap();

    rocket::build()
        .attach(CORS)
        .attach(prometheus.clone())
        .manage(db)
        .manage(config)
        .mount("/", routes![options])
        .mount("/", routes![index])
        .mount("/metrics", prometheus)
        .mount("/auth", routes![
            controllers::auth::sign_in,
            controllers::auth::sign_up,
            controllers::auth::me,
        ],)
        .mount("/authors", routes![
            controllers::authors::index,
            controllers::authors::create,
            controllers::authors::show,
            controllers::authors::update,
            controllers::authors::delete,
            controllers::authors::get_books,
        ],)
        .mount("/books", routes![
            controllers::books::index,
            controllers::books::create,
            controllers::books::show,
            controllers::books::update,
            controllers::books::delete,
        ],)
}
