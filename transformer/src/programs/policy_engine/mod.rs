use crate::{
    error::TransformerError,
    program_handler::{ParseResult, ProgramParser},
    programs::ProgramParseResult,
};
use borsh::BorshDeserialize;
use plerkle_serialization::AccountInfo;
use policy_engine::state::{
    IdentityApproval, PolicyEngine, TransactionAmountLimit, TransactionAmountVelocity,
    TransactionCountVelocity,
};
use solana_sdk::{pubkey::Pubkey, pubkeys};

pubkeys!(
    policy_engine_program_id,
    "6FcM5R2KcdUGcdLunzLm3XLRFr7FiF6Hdz3EWni8YPa2"
);

pub struct PolicyEngineParser;

pub enum PolicyEngineProgram {
    PolicyEngine(Box<PolicyEngine>),
    IdentityApproval(IdentityApproval),
    TransactionAmountLimit(TransactionAmountLimit),
    TransactionAmountVelocity(TransactionAmountVelocity),
    TransactionCountVelocity(TransactionCountVelocity),
    EmptyAccount,
}

impl ParseResult for PolicyEngineProgram {
    fn result(&self) -> &Self
    where
        Self: Sized,
    {
        self
    }
    fn result_type(&self) -> ProgramParseResult {
        ProgramParseResult::PolicyEngineProgram(self)
    }
}

impl ProgramParser for PolicyEngineParser {
    fn key(&self) -> Pubkey {
        policy_engine_program_id()
    }
    fn key_match(&self, key: &Pubkey) -> bool {
        key == &policy_engine_program_id()
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
            return Ok(Box::new(PolicyEngineProgram::EmptyAccount));
        };

        let account_type = match account_data.len() {
            440 => {
                let account_info_without_discriminator = &account_data[8..];
                let account = PolicyEngine::try_from_slice(account_info_without_discriminator)
                    .map_err(|_| {
                        TransformerError::CustomDeserializationError(
                            "Policy Engine Account Unpack Failed".to_string(),
                        )
                    })?;

                PolicyEngineProgram::PolicyEngine(Box::new(account))
            }
            20 => {
                let account_info_without_discriminator = &account_data[8..];
                let account = IdentityApproval::try_from_slice(account_info_without_discriminator)
                    .map_err(|_| {
                        TransformerError::CustomDeserializationError(
                            "Identity Approval Account Unpack Failed".to_string(),
                        )
                    })?;
                PolicyEngineProgram::IdentityApproval(account)
            }
            28 => {
                let account_info_without_discriminator = &account_data[8..];
                let account =
                    TransactionAmountLimit::try_from_slice(account_info_without_discriminator)
                        .map_err(|_| {
                            TransformerError::CustomDeserializationError(
                                "Transaction Amount Limit Account Unpack Failed".to_string(),
                            )
                        })?;
                PolicyEngineProgram::TransactionAmountLimit(account)
            }
            36 => {
                let account_info_without_discriminator = &account_data[8..];
                let account =
                    TransactionAmountVelocity::try_from_slice(account_info_without_discriminator)
                        .map_err(|_| {
                        TransformerError::CustomDeserializationError(
                            "Transaction Amount Velocity Account Unpack Failed".to_string(),
                        )
                    })?;

                PolicyEngineProgram::TransactionAmountVelocity(account)
            }
            36 => {
                let account_info_without_discriminator = &account_data[8..];
                let account =
                    TransactionCountVelocity::try_from_slice(account_info_without_discriminator)
                        .map_err(|_| {
                            TransformerError::CustomDeserializationError(
                                "Transaction Count Velocity Account Unpack Failed".to_string(),
                            )
                        })?;

                PolicyEngineProgram::TransactionCountVelocity(account)
            }
            _ => {
                return Err(TransformerError::InvalidDataLength);
            }
        };

        Ok(Box::new(account_type))
    }
}
