use crate::settings::PHONE_REGEX;
use actix_web::{HttpResponse, error::HttpError};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, FromRow)]
pub struct RegisterUserSchema {
    #[validate(length(min = 1, max = 20))]
    pub user_id: String,
    #[validate(length(min = 3, max = 20))]
    pub first_name: String,
    #[validate(length(min = 3, max = 20))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, max = 32))]
    pub password: String,
    #[validate(
        regex = "PHONE_REGEX",
        // length(min = 9, max = 13, message = "The length of phone should be 9 to 13")
    )]
    pub phone: String,
}

impl RegisterUserSchema {
    pub fn validate_and_respond(&self) -> Result<(), HttpResponse> {
        match self.validate() {
            Ok(_) => Ok(()),
            Err(e) => Err(HttpResponse::UnprocessableEntity().json(e)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterUserForm {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub phone: Option<String>,
}
