use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};
use super::{BaseModel, Pagination};

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct NewAssetModel {
    pub id: String,
    pub parent_id: String,
    pub name: String,
    pub path: String,
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
    FolderTypeMetaData {

    },
    // file type might have a metadata as
    // file_type, file_size
    FileTypeMetaData {
        file_type: String
    }
}

impl MetaDataType {
    pub fn get_file_metadata(&self) -> FileTypeMetaDataStruct {
        match self.to_owned() {
            self::MetaDataType::FileTypeMetaData {file_type} => {
                FileTypeMetaDataStruct {
                    file_type
                }
            },
            MetaDataType::FolderTypeMetaData {} => {
                FileTypeMetaDataStruct::default()
            }
        }
    }
    pub fn get_folder_metadata(&self) -> FolderTypeMetaDataStruct {
        match self.to_owned() {
            MetaDataType::FolderTypeMetaData {} => {
                FolderTypeMetaDataStruct {}
            },
            MetaDataType::FileTypeMetaData {file_type} => {
                let _ = file_type;
                FolderTypeMetaDataStruct::default()
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct FileTypeMetaDataStruct {
    pub file_type: String
}

#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct FolderTypeMetaDataStruct {}

impl Default for MetaDataType {
    fn default() -> MetaDataType {
        MetaDataType::FolderTypeMetaData {}
    }
}

impl TryFrom<Object> for NewAssetModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<NewAssetModel> {
        let id = val.get("id").get_id()?;
        let parent_id = val.get("parent_id").get_string()?;
        let name = val.get("name").get_string()?;
        let path = val.get("path").get_string()?;
        let asset_type = val.get("asset_type").get_string()?;
        let created_at = val.get("created_at").get_datetime()?;
        let created_by = val.get("created_by").get_string()?;
        let updated_at = val.get("updated_at").get_datetime()?;
        let updated_by = val.get("updated_by").get_string()?;

        let metadata = match asset_type.as_str() {
            "FILE" => {
                let object = match val.get("metadata") {
                    Some(val) => {
                        match val.clone() {
                            Value::Object(v) => v,
                            _ => Object::default(),
                        }
                    }
                    None => Object::default(),
                };
                let file_metadata: FileTypeMetaDataStruct = object.try_into()?;
                MetaDataType::FileTypeMetaData {
                    file_type: file_metadata.file_type
                }
            },
            "FOLDER" => {
                // let _object = match val.get("metadata") {
                //     Some(val) => {
                //         match val.clone() {
                //             Value::Object(v) => v,
                //             _ => Object::default(),
                //         }
                //     }
                //     None => Object::default(),
                // };
                // let file_metadata: FolderTypeMetaDataStruct = object.try_into()?;
                MetaDataType::FolderTypeMetaData {}
            },
            _ => MetaDataType::FolderTypeMetaData {}
        };

        Ok(NewAssetModel {
            id,
            parent_id,
            name,
            path,
            asset_type,
            metadata,
            created_at,
            updated_at,
            created_by,
            updated_by,
        })
    }
}

impl TryFrom<Object> for  FileTypeMetaDataStruct {
    type Error = Error;
    fn try_from(val: Object) -> Result<FileTypeMetaDataStruct> {
        let file_type = val.get("file_type").get_string()?;
        Ok(FileTypeMetaDataStruct {
            file_type
        })
    }
}


impl TryFrom<Object> for  FolderTypeMetaDataStruct {
    type Error = Error;
    fn try_from(val: Object) -> Result<FolderTypeMetaDataStruct> {
        // let file_type = val.get("file_type").get_string()?;
        Ok(FolderTypeMetaDataStruct {

        })
    }
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct AssetPagination {
    pub data: Vec<NewAssetModel>,
    pub pagination: Pagination,
}


#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct CreatableAssetModelNew {
    pub logged_in_username: String,
    pub parent_id: String,
    pub name: String,
    pub path: String,
    pub asset_type: String,
    pub metadata: MetaDataType,
}

