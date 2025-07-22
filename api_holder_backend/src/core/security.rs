use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use rand_core::OsRng;

pub fn hash_password(password: &String) -> String {
    let salt = SaltString::generate(&mut OsRng);

    // Create Argon2 instance with default settings
    let argon2 = Argon2::default();

    // Hash the password
    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn verify_password(hashed: &String, password: &String) -> bool {
    let parsed_hash = PasswordHash::new(hashed).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashing_password() {
        let string_password: String = "this_is_a_string_password".to_string();
        let false_string_password: String = "false_string_password".to_string();
        let hashed_password = hash_password(&string_password);
        assert_eq!(verify_password(&hashed_password, &string_password), true);
        assert_eq!(
            verify_password(&hashed_password, &false_string_password),
            false
        );
    }
}
