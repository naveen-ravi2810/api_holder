// endpoints/v1/users.rs
use crate::AppState;
use crate::{
    models::users::{RegisterUserForm, RegisterUserSchema},
    service::users::UserService,
};
use actix_web::{HttpResponse, Responder, web};
use serde_json::json;

pub async fn register_handler(
    form: web::Json<RegisterUserSchema>,
    app_data: web::Data<AppState>,
) -> impl Responder {
    match form.validate_and_respond() {
        Ok(_) => {
            let service: UserService = UserService::new(&app_data.pool).await;
            service.create_user(&form).await;
            HttpResponse::Ok().body("Here is the resp")
        }
        Err(resp) => resp, // sends the full validation errors
    }
}

pub async fn me_handler() -> impl Responder {
    HttpResponse::Ok().body("Here is the resp")
}
