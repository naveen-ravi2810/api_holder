mod constants;
mod models;
mod sql_types;

use actix_web::{App, HttpServer, web};

const ADDRESS: &str = "0.0.0.0:8000";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()).bind(ADDRESS)?.run().await
}
