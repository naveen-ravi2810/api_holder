use actix_web::{App, HttpResponse, HttpServer, Responder, web};

// --- Handler Function ---
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

// --- Main Server ---
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

// --- Tests Section ---
#[cfg(test)]
mod tests {
    use super::*; // Import from main module
    use actix_web::{App, test};

    #[actix_rt::test]
    async fn health_check_returns_200() {
        let app =
            test::init_service(App::new().route("/health_check", web::get().to(health_check)))
                .await;

        let req = test::TestRequest::get().uri("/health_check").to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert_eq!(resp.status(), 200);
    }
}
