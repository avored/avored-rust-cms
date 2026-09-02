use crate::error::Result;

pub trait StringExtension {
    fn is_required(&self) -> Result<bool>;
}

impl StringExtension for String {
    fn is_required(&self) -> Result<bool> {
        if self.is_empty() {
            return Ok(false);
        }
        Ok(true)
    }
}
