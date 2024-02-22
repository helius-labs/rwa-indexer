use crate::{
    error::TransformerError,
    program_handler::{ParseResult, ProgramParser},
    programs::ProgramParseResult,
};
use borsh::BorshDeserialize;
use identity_registry::state::IdentityAccount;
use plerkle_serialization::AccountInfo;
use solana_sdk::{pubkey::Pubkey, pubkeys};

pubkeys!(
    identity_registry_program_id,
    "qDnvwpjBYjH1vs1N1CSdbVkEkePp2acL7TphAYZDeoV"
);

pub struct IdentityRegistryParser;

pub enum IdentityRegistryProgram {
    IdentityAccount(IdentityAccount),
    EmptyAccount,
}

impl ParseResult for IdentityRegistryProgram {
    fn result(&self) -> &Self
    where
        Self: Sized,
    {
        self
    }
    fn result_type(&self) -> ProgramParseResult {
        ProgramParseResult::IdentityRegistryProgram(self)
    }
}

impl ProgramParser for IdentityRegistryParser {
    fn key(&self) -> Pubkey {
        identity_registry_program_id()
    }
    fn key_match(&self, key: &Pubkey) -> bool {
        key == &identity_registry_program_id()
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
            return Ok(Box::new(IdentityRegistryProgram::EmptyAccount));
        };

        let account_type = match account_data.len() {
            83 => {
                let account = IdentityAccount::try_from_slice(&account_data).map_err(|_| {
                    TransformerError::CustomDeserializationError(
                        "Identity Account Unpack Failed".to_string(),
                    )
                })?;

                IdentityRegistryProgram::IdentityAccount(account)
            }
            _ => {
                return Err(TransformerError::InvalidDataLength);
            }
        };

        Ok(Box::new(account_type))
    }
}
