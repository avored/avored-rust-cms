// Usually we don't postfix the extension,
// but naming only string might conflict so we use it.

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};

/// `string_extension`
pub trait StringExtension {
    /// gets the password hash using the provided salt
    fn get_password_hash(&self, password_salt: &str) -> crate::error::Result<String>;

    fn verify_password_hash(&self, password_salt: &str) -> crate::error::Result<bool>;
}

impl StringExtension for String {
    fn get_password_hash(&self, password_salt: &str) -> crate::error::Result<String> {

        let password = self.as_bytes();
        let salt = SaltString::from_b64(password_salt)?;

        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password, &salt)?.to_string();

        Ok(Self::from(password_hash))
    }

    fn verify_password_hash(&self, password_hash: &str) -> crate::error::Result<bool> {

        let password = self.as_bytes();
        let hash = PasswordHash::new(password_hash)?;

        let argon2 = Argon2::default();
        let password_hash = argon2.verify_password(password, &hash);

        Ok(password_hash.is_ok())
    }
}
