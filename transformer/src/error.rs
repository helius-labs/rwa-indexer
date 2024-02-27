use plerkle_serialization::error::PlerkleSerializationError;
use std::io::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransformerError {
    #[error("Instruction Data Parsing Error")]
    InstructionParsingError,
    #[error("IO Error {0}")]
    IOError(String),
    #[error("Could not deserialize data")]
    DeserializationError,
    #[error("Missing Bubblegum event data")]
    MissingBubblegumEventData,
    #[error("Unknown anchor account discriminator.")]
    UnknownAccountDiscriminator,
    #[error("Account type is not valid")]
    InvalidAccountType,
    #[error("Master edition version is invalid")]
    FailedToDeserializeToMasterEdition,
    #[error("Uninitialized account type")]
    UninitializedAccount,
    #[error("Account Type Not implemented")]
    AccountTypeNotImplemented,
    #[error("Could not deserialize data: {0}")]
    CustomDeserializationError(String),
}

impl From<std::io::Error> for TransformerError {
    fn from(err: Error) -> Self {
        TransformerError::IOError(err.to_string())
    }
}

impl From<plerkle_serialization::error::PlerkleSerializationError> for TransformerError {
    fn from(err: PlerkleSerializationError) -> Self {
        TransformerError::IOError(err.to_string())
    }
}
