
use serde::{Deserialize, Serialize};
use std::path::{PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataType {
    IMAGE,
    VIDEO,
    AUDIO,
    UNKNOWN
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: Uuid,
    pub name: String,
    pub data_type: DataType,
    pub data_path: PathBuf
}

impl Asset {

    fn extract_data_type(data_path: PathBuf) -> DataType {
        let extension = data_path.to_str().expect("vidgen-assets-loading-error").split(".").last().expect("vidgen-assets-loading-error");
        if extension == "vidgen-assets-loading-error" {/* Raise error */}

        let mut data_type = DataType::UNKNOWN;
        if include_str!("utils/file_ext/vid.in") == extension {
            data_type = DataType::VIDEO;
        }
        if include_str!("utils/file_ext/img.in") == extension {
            data_type = DataType::IMAGE;
        }
        if include_str!("utils/file_ext/aud.in") == extension {
            data_type = DataType::AUDIO;
        }
        if data_type == DataType::UNKNOWN {
            // Catch error: unsuported file
        }

        data_type

    }

    fn extract_name(data_path: PathBuf) -> String {
        let name = data_path.to_str().expect("vidgen-assets-loading-error").split("/").last().expect("vidgen-assets-loading-error");
        if name == "vidgen-assets-loading-error" {/* Raise error */}
        String::from(name)
    }

    pub fn new(data_path: PathBuf) -> Self {
        
        let name = Self::extract_name(data_path.clone());
        let data_type = Self::extract_data_type(data_path.clone());

        Self {
            id: Uuid::new_v4(),
            name,
            data_type,
            data_path
        }
    }
}
