use crate::{
    error::TransformerError,
    program_handler::{ParseResult, ProgramParser},
    programs::ProgramParseResult,
};
use asset_controller::state::{AssetControllerAccount, TrackerAccount};
use borsh::BorshDeserialize;
use plerkle_serialization::AccountInfo;
use solana_sdk::{pubkey::Pubkey, pubkeys};

use super::get_discriminator;

pubkeys!(
    asset_controller_program_id,
    "DtrBDukceZpUnWmeNzqtoBQPdXW8p9xmWYG1z7qMt8qG"
);

pub struct AssetControllerParser;

pub enum AssetControllerProgram {
    AssetControllerAccount(AssetControllerAccount),
    TrackerAccount(Box<TrackerAccount>),
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

        let asset_controller_discriminator = get_discriminator("AssetControllerAccount");
        let tracker_account_discriminator = get_discriminator("TrackerAccount");
        let account_type_discriminator = &account_data[..8];
        let account_info_without_discriminator = &account_data[8..];

        let account = if account_type_discriminator == asset_controller_discriminator {
            AssetControllerProgram::AssetControllerAccount(
                AssetControllerAccount::try_from_slice(account_info_without_discriminator)
                    .map_err(|_| {
                        TransformerError::CustomDeserializationError(
                            "Failed to deserialize AssetControllerAccount".to_string(),
                        )
                    })?,
            )
        } else if account_type_discriminator == tracker_account_discriminator {
            AssetControllerProgram::TrackerAccount(Box::new(
                TrackerAccount::try_from_slice(account_info_without_discriminator).map_err(
                    |_| {
                        TransformerError::CustomDeserializationError(
                            "Failed to deserialize TrackerAccount".to_string(),
                        )
                    },
                )?,
            ))
        } else {
            return Err(TransformerError::UnknownAccountDiscriminator);
        };
        Ok(Box::new(account))
    }
}
