use argon2::{Argon2, PasswordHasher, PasswordVerifier};

use crate::error::Result;

pub trait StringExtension {
    fn is_required(&self) -> Result<bool>;

    fn get_password_hash(&self, password_salt: &str) -> crate::error::Result<String>;

    fn password_verification(&self, encrypted_password: &str) -> crate::error::Result<bool>;
}

impl StringExtension for String {
    fn is_required(&self) -> Result<bool> {
        if self.is_empty() {
            return Ok(false);
        }
        Ok(true)
    }

    fn get_password_hash(&self, password_salt: &str) -> crate::error::Result<String> {
        let password = self.as_bytes();
        let salt = password_salt.as_bytes();

        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password_with_salt(password, salt)?.to_string();

        Ok(password_hash)
    }

    fn password_verification(&self, encrypted_password: &str) -> crate::error::Result<bool> {
        let password = self.as_bytes();
        let parsed_hash = argon2::PasswordHash::new(encrypted_password)?;
        let argon2 = Argon2::default();
        
        Ok(argon2.verify_password(password, &parsed_hash).is_ok())

    }
}
