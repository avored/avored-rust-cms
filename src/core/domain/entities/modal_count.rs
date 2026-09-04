use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize, Default)]
pub struct ModalCount {
    pub total: u64,
}


#[cfg(feature = "ssr")]
impl TryFrom<surrealdb::types::Object> for ModalCount {
    type Error = crate::error::Error;

    fn try_from(mut obj: surrealdb::types::Object) -> Result<Self, Self::Error> {
       
        let total = match obj.remove("count") {
            Some(surrealdb::types::Value::Number(v)) => v,
            _ => surrealdb::types::Number::Int(0),
        };

        Ok(ModalCount {
            total: total.into_int().unwrap_or(0) as u64,
        })
    }   
}
