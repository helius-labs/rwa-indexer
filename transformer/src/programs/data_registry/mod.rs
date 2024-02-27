use crate::{
    error::TransformerError,
    program_handler::{ParseResult, ProgramParser},
    programs::ProgramParseResult,
};
use borsh::BorshDeserialize;
use data_registry::{state::DataAccount, DataRegistry};
use plerkle_serialization::AccountInfo;
use solana_sdk::{pubkey::Pubkey, pubkeys};

pubkeys!(
    data_registry_program_id,
    "8WRaNVNMDqdwADbKYj7fBd47i2e5SFMSEs8TrA2Vd5io"
);

pub struct DataRegistryParser;

pub enum DataRegistryProgram {
    DataAccount(DataAccount),
    DataRegistry(DataRegistry),
    EmptyAccount,
}

impl ParseResult for DataRegistryProgram {
    fn result(&self) -> &Self
    where
        Self: Sized,
    {
        self
    }
    fn result_type(&self) -> ProgramParseResult {
        ProgramParseResult::DataRegistryProgram(self)
    }
}

impl ProgramParser for DataRegistryParser {
    fn key(&self) -> Pubkey {
        data_registry_program_id()
    }
    fn key_match(&self, key: &Pubkey) -> bool {
        key == &data_registry_program_id()
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
            return Ok(Box::new(DataRegistryProgram::EmptyAccount));
        };

        let account_type = match account_data.len() {
            105 => {
                let account_info_without_discriminator = &account_data[8..];
                let account = DataRegistry::try_from_slice(account_info_without_discriminator)
                    .map_err(|_| {
                        TransformerError::CustomDeserializationError(
                            "Data Registry Unpack Failed".to_string(),
                        )
                    })?;

                DataRegistryProgram::DataRegistry(account)
            }
            105 => {
                let account_info_without_discriminator = &account_data[8..];
                let account = DataAccount::try_from_slice(account_info_without_discriminator)
                    .map_err(|_| {
                        TransformerError::CustomDeserializationError(
                            "Data Account Unpack Failed".to_string(),
                        )
                    })?;

                DataRegistryProgram::DataAccount(account)
            }
            _ => {
                return Err(TransformerError::InvalidDataLength);
            }
        };

        Ok(Box::new(account_type))
    }
}
