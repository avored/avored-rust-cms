use surrealdb::types::{RecordId, RecordIdKey};

pub trait RecordIdExtension {
    fn to_record_id(&self, table: &str) -> RecordId;
}

impl RecordIdExtension for str {
    fn to_record_id(&self, table: &str) -> RecordId {
        let key = self
            .strip_prefix(&format!("{table}:"))
            .unwrap_or(self)
            .to_string();
        RecordId {
            table: table.into(),
            key: RecordIdKey::String(key),
        }
    }
}

impl RecordIdExtension for String {
    fn to_record_id(&self, table: &str) -> RecordId {
        self.as_str().to_record_id(table)
    }
}
