use crate::error::{Error, Result};
use surrealdb::sql::{Array, Object, Value};

pub mod admin_user_model;

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

impl TryFrom<W<Value>> for Object {
	type Error = Error;
	fn try_from(val: W<Value>) -> Result<Object> {
        println!(" i am here try _into");
		match val.0 {
			Value::Object(obj) => Ok(obj),
			_ => Err(Error::XValueNotOfType("Object")),
		}
	}
}
