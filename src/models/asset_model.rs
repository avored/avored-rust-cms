use super::{BaseModel, Pagination};
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct AssetModel {
    pub id: String,
    pub parent_id: String,
    pub name: String,
    pub new_path: String,
    pub asset_type: String,
    pub metadata: MetaDataType,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MetaDataType {
    // values can be folder color or no of files
    FolderTypeMetaData { color: String },
    // file type might have a metadata as
    // file_type, file_size
    FileTypeMetaData { file_type: String },
}

impl MetaDataType {
    pub fn get_file_metadata(&self) -> FileTypeMetaDataStruct {
        match self.to_owned() {
            MetaDataType::FileTypeMetaData { file_type } => FileTypeMetaDataStruct { file_type },
            MetaDataType::FolderTypeMetaData { color } => {
                let _ = color;
                FileTypeMetaDataStruct::default()
            }
        }
    }
    pub fn get_folder_metadata(&self) -> FolderTypeMetaDataStruct {
        match self.to_owned() {
            MetaDataType::FolderTypeMetaData { color } => FolderTypeMetaDataStruct { color },
            MetaDataType::FileTypeMetaData { file_type } => {
                let _ = file_type;
                FolderTypeMetaDataStruct::default()
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct FileTypeMetaDataStruct {
    pub file_type: String,
}

#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct FolderTypeMetaDataStruct {
    pub color: String,
}

impl Default for MetaDataType {
    fn default() -> MetaDataType {
        MetaDataType::FolderTypeMetaData {
            color: String::from(""),
        }
    }
}

impl TryFrom<Object> for AssetModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<AssetModel> {
        let id = val.get("id").get_id()?;
        let parent_id = val.get("parent_id").get_string()?;
        let name = val.get("name").get_string()?;
        let asset_type = val.get("asset_type").get_string()?;
        let created_at = val.get("created_at").get_datetime()?;
        let created_by = val.get("created_by").get_string()?;
        let updated_at = val.get("updated_at").get_datetime()?;
        let updated_by = val.get("updated_by").get_string()?;

        let metadata = match asset_type.as_str() {
            "FILE" => {
                let object = match val.get("metadata") {
                    Some(val) => match val.clone() {
                        Value::Object(v) => v,
                        _ => Object::default(),
                    },
                    None => Object::default(),
                };
                let file_metadata: FileTypeMetaDataStruct = object.try_into()?;
                MetaDataType::FileTypeMetaData {
                    file_type: file_metadata.file_type,
                }
            }
            "FOLDER" => {
                let object = match val.get("metadata") {
                    Some(val) => match val.clone() {
                        Value::Object(v) => v,
                        _ => Object::default(),
                    },
                    None => Object::default(),
                };
                let folder_metadata: FolderTypeMetaDataStruct = object.try_into()?;
                MetaDataType::FolderTypeMetaData {
                    color: folder_metadata.color,
                }
            }
            _ => MetaDataType::FolderTypeMetaData {
                color: String::from("text-gray-400"),
            },
        };

        let new_path = String::from("");

        Ok(AssetModel {
            id,
            parent_id,
            name,
            new_path,
            asset_type,
            metadata,
            created_at,
            updated_at,
            created_by,
            updated_by,
        })
    }
}

impl TryFrom<Object> for FileTypeMetaDataStruct {
    type Error = Error;
    fn try_from(val: Object) -> Result<FileTypeMetaDataStruct> {
        let file_type = val.get("file_type").get_string()?;
        Ok(FileTypeMetaDataStruct { file_type })
    }
}

impl TryFrom<Object> for FolderTypeMetaDataStruct {
    type Error = Error;
    fn try_from(val: Object) -> Result<FolderTypeMetaDataStruct> {
        let color = val.get("color").get_string()?;
        Ok(FolderTypeMetaDataStruct { color })
    }
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct AssetPagination {
    pub data: Vec<AssetModel>,
    pub pagination: Pagination,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct CreatableAssetModel {
    pub logged_in_username: String,
    pub parent_id: String,
    pub name: String,
    pub asset_type: String,
    pub metadata: MetaDataType,
}
