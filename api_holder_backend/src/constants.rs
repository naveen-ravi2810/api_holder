use lazy_static::lazy_static;
use std::env;
lazy_static! {
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: i16 = set_port();
}

fn set_address() -> String {
    dotenv::dotenv().ok();
    env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string())
}

fn set_port() -> i16 {
    dotenv::dotenv().ok();
    env::var("PORT")
        .unwrap_or_else(|_| "i16".to_string())
        .parse::<i16>()
        .unwrap()
}
