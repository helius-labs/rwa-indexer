use crate::{
    error::TransformerError,
    program_handler::{ParseResult, ProgramParser},
    programs::ProgramParseResult,
};
use borsh::BorshDeserialize;
use plerkle_serialization::AccountInfo;
use policy_engine::state::PolicyAccount;
use solana_sdk::{pubkey::Pubkey, pubkeys};

use super::get_discriminator;

pubkeys!(
    policy_engine_program_id,
    "6FcM5R2KcdUGcdLunzLm3XLRFr7FiF6Hdz3EWni8YPa2"
);

pub struct PolicyEngineParser;

pub enum PolicyEngineProgram {
    PolicyAccount(Box<PolicyAccount>),
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

        let policy_engine_descriminator = get_discriminator("PolicyAccount");
        let account_type_discriminator = &account_data[..8];
        let account_info_without_discriminator = &account_data[8..];

        let account = if account_type_discriminator == policy_engine_descriminator {
            let account = PolicyAccount::try_from_slice(account_info_without_discriminator)
                .map_err(|_| {
                    TransformerError::CustomDeserializationError(
                        "Policy Engine Account Unpack Failed".to_string(),
                    )
                })?;
            PolicyEngineProgram::PolicyAccount(Box::new(account))
        } else {
            return Err(TransformerError::UnknownAccountDiscriminator);
        };

        Ok(Box::new(account))
    }
}
