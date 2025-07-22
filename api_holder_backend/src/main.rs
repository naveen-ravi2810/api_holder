mod core;
mod endpoints;
mod models;
mod service;
mod settings;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};

use sqlx;
use sqlx::{Pool, Postgres};

pub struct AppState {
    pool: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }
    env_logger::init();

    let pool = core::db_conn::get_pool_connection().await;

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { pool: pool.clone() }))
            .configure(endpoints::v1::config)
            .wrap(Logger::default())
    })
    .bind((settings::HOST.as_str(), *settings::PORT))?
    .run()
    .await
}
