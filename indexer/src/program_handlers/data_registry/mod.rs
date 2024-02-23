use crate::{config::IndexerConfig, error::IndexerError, metric};
use cadence_macros::{is_global_default_set, statsd_count};
use plerkle_serialization::AccountInfo;
use sea_orm::{
    entity::*, query::*, sea_query::OnConflict, ActiveValue::Set, ConnectionTrait,
    DatabaseConnection, DbBackend, EntityTrait,
};
use solana_sdk::program_option::COption;
use spl_token::state::AccountState;
use tokio::sync::mpsc::UnboundedSender;
use transformer::programs::data_registry::DataRegistryProgram;

pub async fn handle_data_registry_program_account<'a, 'b, 'c>(
    account_update: &'a AccountInfo<'a>,
    parsing_result: &'b DataRegistryProgram,
    db: &'c DatabaseConnection,
    _config: &IndexerConfig,
) -> Result<(), IndexerError> {
    let key = *account_update.pubkey().unwrap();
    let key_bytes = key.0.to_vec();
    let spl_token_program = account_update.owner().unwrap().0.to_vec();
    match &parsing_result {
        DataRegistryProgram::DataAccount(da) => Ok(()),
        _ => Err(IndexerError::NotImplemented),
    }?;
    Ok(())
}
