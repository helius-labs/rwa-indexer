use crate::{
    error::TransformerError,
    program_handler::{ParseResult, ProgramParser},
    programs::ProgramParseResult,
};
use borsh::BorshDeserialize;
use data_registry::{state::DataAccount, DataRegistryAccount};
use plerkle_serialization::AccountInfo;
use solana_sdk::{pubkey::Pubkey, pubkeys};

use super::get_discriminator;

pubkeys!(
    data_registry_program_id,
    "dataeP5X1e7XsWN1ovDSEDP5cqaEUnKBmHE5iZhXPVw"
);

pub struct DataRegistryParser;

pub enum DataRegistryProgram {
    DataAccount(DataAccount),
    DataRegistry(DataRegistryAccount),
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

        let data_registry_discriminator = get_discriminator("DataRegistryAccount");
        let data_account_discriminator = get_discriminator("DataAccount");
        let account_type_discriminator = &account_data[..8];
        let account_info_without_discriminator = &account_data[8..];

        let account = if account_type_discriminator == data_registry_discriminator {
            let account = DataRegistryAccount::try_from_slice(account_info_without_discriminator)
                .map_err(|_| {
                TransformerError::CustomDeserializationError(
                    "Data Registry Unpack Failed".to_string(),
                )
            })?;

            DataRegistryProgram::DataRegistry(account)
        } else if account_type_discriminator == data_account_discriminator {
            let account =
                DataAccount::try_from_slice(account_info_without_discriminator).map_err(|_| {
                    TransformerError::CustomDeserializationError(
                        "Data Account Unpack Failed".to_string(),
                    )
                })?;

            DataRegistryProgram::DataAccount(account)
        } else {
            return Err(TransformerError::UnknownAccountDiscriminator);
        };
        Ok(Box::new(account))
    }
}
