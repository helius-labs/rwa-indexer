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

use super::get_discriminator;

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

        let policy_engine_descriminator = get_discriminator("PolicyEngine");
        let identity_approval_descriminator = get_discriminator("IdentityApproval");
        let transaction_amount_limit_descriminator = get_discriminator("TransactionAmountLimit");
        let transaction_amount_velocity_descriminator =
            get_discriminator("TransactionAmountVelocity");
        let transaction_count_velocity_descriminator =
            get_discriminator("TransactionCountVelocity");
        let account_type_discriminator = &account_data[..8];
        let account_info_without_discriminator = &account_data[8..];

        let account = if account_type_discriminator == policy_engine_descriminator {
            let account = PolicyEngine::try_from_slice(account_info_without_discriminator)
                .map_err(|_| {
                    TransformerError::CustomDeserializationError(
                        "Policy Engine Account Unpack Failed".to_string(),
                    )
                })?;
            PolicyEngineProgram::PolicyEngine(Box::new(account))
        } else if account_type_discriminator == identity_approval_descriminator {
            let account = IdentityApproval::try_from_slice(account_info_without_discriminator)
                .map_err(|_| {
                    TransformerError::CustomDeserializationError(
                        "Identity Approval Account Unpack Failed".to_string(),
                    )
                })?;
            PolicyEngineProgram::IdentityApproval(account)
        } else if account_type_discriminator == transaction_amount_limit_descriminator {
            let account =
                TransactionAmountLimit::try_from_slice(account_info_without_discriminator)
                    .map_err(|_| {
                        TransformerError::CustomDeserializationError(
                            "Transaction Amount Limit Account Unpack Failed".to_string(),
                        )
                    })?;
            PolicyEngineProgram::TransactionAmountLimit(account)
        } else if account_type_discriminator == transaction_amount_velocity_descriminator {
            let account =
                TransactionAmountVelocity::try_from_slice(account_info_without_discriminator)
                    .map_err(|_| {
                        TransformerError::CustomDeserializationError(
                            "Transaction Amount Velocity Account Unpack Failed".to_string(),
                        )
                    })?;
            PolicyEngineProgram::TransactionAmountVelocity(account)
        } else if account_type_discriminator == transaction_count_velocity_descriminator {
            let account_info_without_discriminator = &account_data[8..];
            let account =
                TransactionCountVelocity::try_from_slice(account_info_without_discriminator)
                    .map_err(|_| {
                        TransformerError::CustomDeserializationError(
                            "Transaction Count Velocity Account Unpack Failed".to_string(),
                        )
                    })?;
            PolicyEngineProgram::TransactionCountVelocity(account)
        } else {
            return Err(TransformerError::UnknownAccountDiscriminator);
        };

        Ok(Box::new(account))
    }
}
