use crate::error::StorageError;
use serde::{de::DeserializeOwned, Serialize};
use std::{fs, path::Path};

pub fn save_json<T: Serialize>(path: &Path, value: &T) -> Result<(), StorageError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, serde_json::to_string_pretty(value)?)?;
    Ok(())
}

pub fn load_json<T: DeserializeOwned>(path: &Path) -> Result<T, StorageError> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}