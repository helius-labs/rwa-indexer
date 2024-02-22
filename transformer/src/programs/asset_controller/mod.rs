use crate::{
    error::TransformerError,
    program_handler::{ParseResult, ProgramParser},
    programs::ProgramParseResult,
};
use asset_controller::state::{AssetControllerAccount, TrackerAccount, TransactionApprovalAccount};
use plerkle_serialization::AccountInfo;
use solana_sdk::{program_pack::Pack, pubkey::Pubkey, pubkeys};
use spl_token::state::{Account as TokenAccount, Mint};

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
                let asset_controller =
                    AssetControllerAccount::unpack(&account_data).map_err(|_| {
                        TransformerError::CustomDeserializationError(
                            "Asset Controller Account Unpack Failed".to_string(),
                        )
                    })?;

                AssetControllerProgram::AssetControllerAccount(asset_controller)
            }
            136 => {
                let asset_controller =
                    TransactionApprovalAccount::unpack(&account_data).map_err(|_| {
                        TransformerError::CustomDeserializationError(
                            "Transaction Approval Account Unpack Failed".to_string(),
                        )
                    })?;

                AssetControllerProgram::AssetControllerAccount(asset_controller)
            }
            240 => {
                let asset_controller = TrackerAccount::unpack(&account_data).map_err(|_| {
                    TransformerError::CustomDeserializationError(
                        "Transaction Approval Account Unpack Failed".to_string(),
                    )
                })?;

                AssetControllerProgram::TrackerAccount(asset_controller)
            }
            _ => {
                return Err(TransformerError::InvalidDataLength);
            }
        };

        Ok(Box::new(account_type))
    }
}
