use super::{BaseModel, Pagination};
use crate::error::{Error, Result};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::SystemTime;
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

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct FileTypeMetaData {
    pub file_type: String,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct FolderTypeMetaData {
    pub color: String,
}

// { color: String }
//FileTypeMetaData { file_type: String }
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct MetaDataType {
    // values can be folder color or no of files
    pub file_meta_data: FileTypeMetaData,
    // file type might have a metadata as
    // file_type, file_size
    pub folder_meta_data: FolderTypeMetaData,
}

// impl MetaDataType {
//     pub fn get_file_metadata(&self) -> FileTypeMetaDataStruct {
//         match self.to_owned() {
//             MetaDataType::FileTypeMetaData { file_type } => FileTypeMetaDataStruct { file_type },
//             MetaDataType::FolderTypeMetaData { color } => {
//                 let _ = color;
//                 FileTypeMetaDataStruct::default()
//             }
//         }
//     }
//     pub fn get_folder_metadata(&self) -> FolderTypeMetaDataStruct {
//         match self.to_owned() {
//             MetaDataType::FolderTypeMetaData { color } => FolderTypeMetaDataStruct { color },
//             MetaDataType::FileTypeMetaData { file_type } => {
//                 let _ = file_type;
//                 FolderTypeMetaDataStruct::default()
//             }
//         }
//     }
// }

#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct FileTypeMetaDataStruct {
    pub file_type: String,
}

#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct FolderTypeMetaDataStruct {
    pub color: String,
}

// impl Default for MetaDataType {
//     fn default() -> MetaDataType {
//         MetaDataType::FolderTypeMetaData {
//             color: String::from(""),
//         }
//     }
// }

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

        let metadata = MetaDataType::default();
        // let metadata = match asset_type.as_str() {
        //     "FILE" => {
        //         let object = match val.get("metadata") {
        //             Some(val) => match val.clone() {
        //                 Value::Object(v) => v,
        //                 _ => Object::default(),
        //             },
        //             None => Object::default(),
        //         };
        //         let file_metadata: FileTypeMetaDataStruct = object.try_into()?;
        //         // MetaDataType::FileTypeMetaData {
        //         //     file_type: file_metadata.file_type,
        //         // }
        //     }
        //     "FOLDER" => {
        //         let object = match val.get("metadata") {
        //             Some(val) => match val.clone() {
        //                 Value::Object(v) => v,
        //                 _ => Object::default(),
        //             },
        //             None => Object::default(),
        //         };
        //         let folder_metadata: FolderTypeMetaDataStruct = object.try_into()?;
        //         // MetaDataType::FolderTypeMetaData {
        //         //     color: folder_metadata.color,
        //         // }
        //     }
        //     _ => MetaDataType::default(),
        //     },
        // };

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

impl TryFrom<AssetModel> for crate::api::proto::asset::AssetModel {
    type Error = Error;

    fn try_from(val: AssetModel) -> Result<crate::api::proto::asset::AssetModel> {
        let chrono_utc_created_at = val.created_at.to_utc();
        let system_time_created_at = SystemTime::from(chrono_utc_created_at);
        let created_at = Timestamp::from(system_time_created_at);

        let chrono_utc_updated_at = val.updated_at.to_utc();
        let system_time_updated_at = SystemTime::from(chrono_utc_updated_at);
        let updated_at = Timestamp::from(system_time_updated_at);

        let metadata = val.metadata.try_into()?;

        let model: crate::api::proto::asset::AssetModel = crate::api::proto::asset::AssetModel {
            id: val.id,
            name: val.name,
            new_path: val.new_path,
            asset_type: val.asset_type,
            parent_id: Some(val.parent_id),
            created_at: Option::from(created_at),
            updated_at: Option::from(updated_at),
            created_by: val.created_by,
            updated_by: val.updated_by,
            metadata: Some(metadata),
        };

        Ok(model)
    }
}

impl TryFrom<FileTypeMetaData> for crate::api::proto::asset::FileTypeMetaData {
    type Error = Error;
    fn try_from(val: FileTypeMetaData) -> Result<crate::api::proto::asset::FileTypeMetaData> {
        let model: crate::api::proto::asset::FileTypeMetaData =
            crate::api::proto::asset::FileTypeMetaData {
                file_type: val.file_type,
            };
        Ok(model)
    }
}

impl TryFrom<FolderTypeMetaData> for crate::api::proto::asset::FolderTypeMetaData {
    type Error = Error;
    fn try_from(val: FolderTypeMetaData) -> Result<crate::api::proto::asset::FolderTypeMetaData> {
        let model: crate::api::proto::asset::FolderTypeMetaData =
            crate::api::proto::asset::FolderTypeMetaData { color: val.color };
        Ok(model)
    }
}

impl TryFrom<MetaDataType> for crate::api::proto::asset::MetaDataType {
    type Error = Error;
    fn try_from(val: MetaDataType) -> Result<crate::api::proto::asset::MetaDataType> {
        let model: crate::api::proto::asset::MetaDataType =
            crate::api::proto::asset::MetaDataType {
                file_meta_data: Some(val.file_meta_data.try_into()?),
                folder_meta_data: Some(val.folder_meta_data.try_into()?),
            };
        Ok(model)
    }
}

impl TryFrom<FolderTypeMetaData> for Value {
    type Error = Error;
    fn try_from(val: FolderTypeMetaData) -> Result<Value> {
        let val_val: BTreeMap<String, Value> = [("color".into(), val.color.into())].into();

        Ok(val_val.into())
    }
}

impl TryFrom<FileTypeMetaData> for Value {
    type Error = Error;
    fn try_from(val: FileTypeMetaData) -> Result<Value> {
        let val_val: BTreeMap<String, Value> = [("file_type".into(), val.file_type.into())].into();

        Ok(val_val.into())
    }
}

impl TryFrom<MetaDataType> for Value {
    type Error = Error;
    fn try_from(val: MetaDataType) -> Result<Value> {
        let val_val: BTreeMap<String, Value> = [
            ("folder_meta_data".into(), val.folder_meta_data.try_into()?),
            ("file_meta_data".into(), val.file_meta_data.try_into()?),
        ]
        .into();

        Ok(val_val.into())
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
