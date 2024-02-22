use crate::{
    error::TransformerError,
    program_handler::{ParseResult, ProgramParser},
    programs::ProgramParseResult,
};
use asset_controller::state::{AssetControllerAccount, TrackerAccount, TransactionApprovalAccount};
use borsh::BorshDeserialize;
use plerkle_serialization::AccountInfo;
use solana_sdk::{pubkey::Pubkey, pubkeys};

pubkeys!(
    asset_controller_program_id,
    "DtrBDukceZpUnWmeNzqtoBQPdXW8p9xmWYG1z7qMt8qG"
);

pub struct AssetControllerParser;

pub enum AssetControllerProgram {
    AssetControllerAccount(AssetControllerAccount),
    TransactionApprovalAccount(TransactionApprovalAccount),
    TrackerAccount(TrackerAccount),
    EmptyAccount,
}

impl ParseResult for AssetControllerProgram {
    fn result(&self) -> &Self
    where
        Self: Sized,
    {
        self
    }
    fn result_type(&self) -> ProgramParseResult {
        ProgramParseResult::AssetControllerProgram(self)
    }
}

impl ProgramParser for AssetControllerParser {
    fn key(&self) -> Pubkey {
        asset_controller_program_id()
    }
    fn key_match(&self, key: &Pubkey) -> bool {
        key == &asset_controller_program_id()
    }
    fn handles_account_updates(&self) -> bool {
        true
    }

    fn handles_instructions(&self) -> bool {
        false
    }
    fn handle_account(
        &self,
        account_info: &AccountInfo,
    ) -> Result<Box<(dyn ParseResult + 'static)>, TransformerError> {
        let account_data = if let Some(account_info) = account_info.data() {
            account_info.iter().collect::<Vec<_>>()
        } else {
            return Ok(Box::new(AssetControllerProgram::EmptyAccount));
        };

        let account_type = match account_data.len() {
            105 => {
                let account =
                    AssetControllerAccount::try_from_slice(&account_data).map_err(|_| {
                        TransformerError::CustomDeserializationError(
                            "Asset Controller Account Unpack Failed".to_string(),
                        )
                    })?;

                AssetControllerProgram::AssetControllerAccount(account)
            }
            136 => {
                let account =
                    TransactionApprovalAccount::try_from_slice(&account_data).map_err(|_| {
                        TransformerError::CustomDeserializationError(
                            "Transaction Approval Account Unpack Failed".to_string(),
                        )
                    })?;

                AssetControllerProgram::TransactionApprovalAccount(account)
            }
            240 => {
                let account = TrackerAccount::try_from_slice(&account_data).map_err(|_| {
                    TransformerError::CustomDeserializationError(
                        "Transaction Approval Account Unpack Failed".to_string(),
                    )
                })?;

                AssetControllerProgram::TrackerAccount(account)
            }
            _ => {
                return Err(TransformerError::InvalidDataLength);
            }
        };

        Ok(Box::new(account_type))
    }
}
