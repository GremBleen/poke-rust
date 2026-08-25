#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SyncError {
    #[error("PokeAPI request failed: {0}")]
    Api(#[from] rustemon::error::Error),
    #[error(transparent)]
    Storage(#[from] StorageError),
}