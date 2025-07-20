// Usually we don't postfix the extension,
// but naming only string might conflict so we use it.
pub trait StringExtension {
    fn get_password_hash(&self, password_salt: &str) -> crate::error::Result<String>;
}

impl StringExtension for String {
    fn get_password_hash(&self, password_salt: &str) -> crate::error::Result<String> {
        Ok(String::from(password_salt))
    }
}
