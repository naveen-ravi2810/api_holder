use lazy_static::lazy_static;
use regex::Regex;
use std::env;

lazy_static! {
    pub static ref PORT: u16 = get_port();
    pub static ref HOST: String = get_host();
    pub static ref PHONE_REGEX: Regex = Regex::new(r"^\+?[0-9]{10,15}$").unwrap();
}

fn get_port() -> u16 {
    dotenv::dotenv().ok();
    env::var("PORT")
        .ok()
        .and_then(|val| val.parse::<u16>().ok())
        .unwrap_or(8080)
}
fn get_host() -> String {
    dotenv::dotenv().ok();
    std::env::var("ADDRESS").unwrap_or("0.0.0.0".to_string())
}
