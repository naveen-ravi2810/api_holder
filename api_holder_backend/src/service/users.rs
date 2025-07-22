use crate::core::security::hash_password;
use crate::models::users::RegisterUserSchema;
use sqlx::postgres::PgPool;
pub struct UserService {
    pub pool: PgPool,
}
impl UserService {
    pub async fn new(pool: &PgPool) -> Self {
        UserService { pool: pool.clone() }
    }

    pub async fn create_user(&self, new_user: &RegisterUserSchema) -> () {
        // let query_result = sqlx::query_as!(
        //     RegisterUserSchema,
        //     "INSERT INTO users (user_id, first_name, last_name, email, phone, password) values ($1,$2,$3,$4,$5,$6) RETURNING user_id, first_name, last_name, email, phone, password",
        //     "nav123",
        //     new_user.first_name,
        //     new_user.last_name,
        //     new_user.email,
        //     new_user.phone,
        //     hash_password(&new_user.password)
        // ).fetch_one(&self.pool).await;
        // match query_result {
        //     Ok(_) => {
        //         println!("It is ok");
        //     }
        //     Err(e) => {
        //         println!("{:?}", e.as_database_error())
        //     }
        // }
    }
}
