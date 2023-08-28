use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Array, Object, Value, Datetime, Number};

pub mod admin_user_model;
pub mod role_model;

pub struct W<T>(pub T);

impl TryFrom<W<Value>> for String {
	type Error = Error;
	fn try_from(val: W<Value>) -> Result<String> {
		match val.0 {
			Value::Strand(strand) => Ok(strand.as_string()),
			Value::Thing(thing) => Ok(thing.to_string()),
			_ => Err(Error::XValueNotOfType("String")),
		}
	}
}

impl TryFrom<W<Value>> for Array {
	type Error = Error;
	fn try_from(val: W<Value>) -> Result<Array> {
		match val.0 {
			Value::Array(obj) => Ok(obj),
			_ => Err(Error::XValueNotOfType("Array")),
		}
	}
}


impl TryFrom<W<Value>> for i64 {
	type Error = Error;
	fn try_from(val: W<Value>) -> Result<i64> {
		match val.0 {
			Value::Number(obj) => Ok(obj.as_int()),
			_ => Err(Error::XValueNotOfType("i64")),
		}
	}
}

impl TryFrom<W<Value>> for bool {
	type Error = Error;
	fn try_from(val: W<Value>) -> Result<bool> {
		match val.0 {
			Value::False => Ok(false),
			Value::True => Ok(true),
			_ => Err(Error::XValueNotOfType("bool")),
		}
	}
}


impl TryFrom<W<Value>> for Datetime {
	type Error = Error;
	fn try_from(val: W<Value>) -> Result<Datetime> {
		match val.0 {
			Value::Datetime(obj) => Ok(obj),
			_ => Err(Error::XValueNotOfType("datetime")),
		}
	}
}

impl TryFrom<W<Value>> for Number {
	type Error = Error;
	fn try_from(val: W<Value>) -> Result<Number> {
		match val.0 {
			Value::Number(obj) => Ok(obj),
			_ => Err(Error::XValueNotOfType("number")),
		}
	}
}


impl TryFrom<W<Value>> for Object {
	type Error = Error;
	fn try_from(val: W<Value>) -> Result<Object> {
		match val.0 {
			Value::Object(obj) => Ok(obj),
			_ => Err(Error::XValueNotOfType("Object")),
		}
	}
}




#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct ModelCount {
    pub count: i64
}
impl ModelCount {
    pub fn new() -> Self {
        ModelCount {
            count: 0
        }
    }
}

impl TryFrom<Object> for ModelCount {
    type Error = Error;
    fn try_from(val: Object) -> Result<ModelCount> {
        let count = match val.get("count") {
            Some(val) => val.clone(),
            None => Value::Null,
        };
        let mut model_count = ModelCount::new();
        model_count.count = count.as_int();

        Ok(model_count)
    }
}
