use log::{debug, error, info};
use sea_orm::DbErr;

use {jsonrpsee::core::Error as RpcError, jsonrpsee::types::error::CallError, thiserror::Error};

#[derive(Error, Debug)]
pub enum RwaApiError {
    #[error("Config Missing or Error: {0}")]
    ConfigurationError(String),
    #[error("Server Failed to Start")]
    ServerStartError(#[from] RpcError),
    #[error("Database Connection Failed")]
    DatabaseConnectionError(#[from] sqlx::Error),
    #[error("Pubkey Validation Err: {0} is invalid")]
    PubkeyValidationError(String),
    #[error("Validation Error: {0}")]
    ValidationError(String),
    #[error("Database Error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("Pagination Error. Only one pagination parameter supported per query.")]
    PaginationError,
    #[error("Pagination Error. No Pagination Method Selected.")]
    PaginationEmptyError,
    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] serde_json::Error),
    #[error("Pagination Error. Limit should not be greater than 1000.")]
    PaginationExceededError,
    #[error("Batch Size Error. Batch size should not be greater than 1000.")]
    BatchSizeExceededError,
    #[error("Pagination Sorting Error. Only sorting based on id is supported for this pagination option.")]
    PaginationSortingValidationError,
}

impl From<RwaApiError> for RpcError {
    fn from(val: RwaApiError) -> Self {
        match &val {
            RwaApiError::ValidationError(_) => {
                debug!("{}", val);
            }
            RwaApiError::DatabaseError(e) => match e {
                DbErr::RecordNotFound(_) => {
                    debug!("{}", e);
                }
                _ => {
                    error!("{}", e);
                }
            },
            RwaApiError::DatabaseConnectionError(_)
            | RwaApiError::ConfigurationError(_)
            | RwaApiError::DeserializationError(_)
            | RwaApiError::ServerStartError(_) => {
                error!("{}", val);
            }
            _ => {
                info!("{}", val);
            }
        }
        RpcError::Call(CallError::from_std_error(val))
    }
}
