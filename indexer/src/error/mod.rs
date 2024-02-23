use plerkle_messenger::MessengerError;
use plerkle_serialization::error::PlerkleSerializationError;
use sea_orm::{DbErr, TransactionError};
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;
use transformer::error::TransformerError;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum IndexerError {
    #[error("ChangeLog Event Malformed")]
    ChangeLogEventMalformed,
    #[error("Compressed Asset Event Malformed")]
    CompressedAssetEventMalformed,
    #[error("Network Error: {0}")]
    BatchInitNetworkingError(String),
    #[error("Error writing batch files")]
    BatchInitIOError,
    #[error("Storage listener error: ({msg})")]
    StorageListenerError { msg: String },
    #[error("Storage Write Error: {0}")]
    StorageWriteError(String),
    #[error("NotImplemented")]
    NotImplemented,
    #[error("Deserialization Error: {0}")]
    DeserializationError(String),
    #[error("Missing or invalid configuration: ({msg})")]
    ConfigurationError { msg: String },
    #[error("Error getting RPC data: {0}")]
    RpcGetDataError(String),
    #[error("RPC returned data in unsupported format: {0}")]
    RpcDataUnsupportedFormat(String),
    #[error("Data serializaton error: {0}")]
    SerializatonError(String),
    #[error("Messenger error; {0}")]
    MessengerError(String),
    #[error("Transformer Parsing error: {0}")]
    ParsingError(String),
    #[error("Database Error: {0}")]
    DatabaseError(String),
    #[error("Cache Storage Write Error: {0}")]
    CacheStorageWriteError(String),
    #[error("HttpError {status_code}")]
    HttpError { status_code: String, uri: String },
    #[error("AssetIndex Error {0}")]
    AssetIndexError(String),
}

impl From<reqwest::Error> for IndexerError {
    fn from(err: reqwest::Error) -> Self {
        IndexerError::BatchInitNetworkingError(err.to_string())
    }
}

impl From<stretto::CacheError> for IndexerError {
    fn from(err: stretto::CacheError) -> Self {
        IndexerError::CacheStorageWriteError(err.to_string())
    }
}

impl From<serde_json::Error> for IndexerError {
    fn from(_err: serde_json::Error) -> Self {
        IndexerError::SerializatonError("JSON ERROR".to_string())
    }
}

impl From<TransformerError> for IndexerError {
    fn from(err: TransformerError) -> Self {
        IndexerError::ParsingError(err.to_string())
    }
}

impl From<std::io::Error> for IndexerError {
    fn from(_err: std::io::Error) -> Self {
        IndexerError::BatchInitIOError
    }
}

impl From<DbErr> for IndexerError {
    fn from(e: DbErr) -> Self {
        IndexerError::StorageWriteError(e.to_string())
    }
}

impl From<TransactionError<IndexerError>> for IndexerError {
    fn from(e: TransactionError<IndexerError>) -> Self {
        IndexerError::StorageWriteError(e.to_string())
    }
}

impl From<MessengerError> for IndexerError {
    fn from(e: MessengerError) -> Self {
        IndexerError::MessengerError(e.to_string())
    }
}

impl From<PlerkleSerializationError> for IndexerError {
    fn from(e: PlerkleSerializationError) -> Self {
        IndexerError::SerializatonError(e.to_string())
    }
}
