use crate::{config::IndexerConfig, error::IndexerError};
use plerkle_serialization::AccountInfo;
use rwa_types::dao::{
    data_account, data_registry,
    sea_orm_active_enums::{DataAccountType, DataRegistryVersion},
};
use sea_orm::{
    query::*, sea_query::OnConflict, ActiveValue::Set, ConnectionTrait, DatabaseConnection,
    DbBackend, EntityTrait,
};
use transformer::programs::data_registry::DataRegistryProgram;

pub async fn handle_data_registry_program_account<'a, 'b, 'c>(
    account_update: &'a AccountInfo<'a>,
    parsing_result: &'b DataRegistryProgram,
    db: &'c DatabaseConnection,
    _config: &IndexerConfig,
) -> Result<(), IndexerError> {
    let key = *account_update.pubkey().unwrap();
    let key_bytes = key.0.to_vec();
    match &parsing_result {
        DataRegistryProgram::DataRegistry(dr) => {
            let active_model = data_registry::ActiveModel {
                id: Set(key_bytes.clone()),
                asset_mint: Set(dr.asset_mint.to_bytes().to_vec()),
                authority: Set(dr.authority.to_bytes().to_vec()),
                version: Set(DataRegistryVersion::from(dr.version)),
                slot_updated: Set(account_update.slot() as i64),
                ..Default::default()
            };

            let mut query = data_registry::Entity::insert(active_model)
                .on_conflict(
                    OnConflict::columns([data_registry::Column::Id])
                        .update_columns([
                            data_registry::Column::AssetMint,
                            data_registry::Column::Authority,
                            data_registry::Column::Version,
                            data_registry::Column::SlotUpdated,
                        ])
                        .to_owned(),
                )
                .build(DbBackend::Postgres);

            query.sql = format!(
                "{} WHERE excluded.slot_updated >= data_registry.slot_updated OR data_registry.slot_updated IS NULL",
                query.sql);

            let txn = db.begin().await?;
            txn.execute(query)
                .await
                .map_err(|db_err| IndexerError::AssetIndexError(db_err.to_string()))?;
            txn.commit().await?;

            Ok(())
        }
        DataRegistryProgram::DataAccount(da) => {
            let active_model = data_account::ActiveModel {
                id: Set(key_bytes.clone()),
                name: Set(da.name.clone()),
                uri: Set(da.uri.clone()),
                data_registry: Set(da.data_registry.to_bytes().to_vec()),
                data_type: Set(DataAccountType::from(da._type.clone())),
                slot_updated: Set(account_update.slot() as i64),
                ..Default::default()
            };

            let mut query = data_account::Entity::insert(active_model)
                .on_conflict(
                    OnConflict::columns([data_account::Column::Id])
                        .update_columns([
                            data_account::Column::Name,
                            data_account::Column::Uri,
                            data_account::Column::DataRegistry,
                            data_account::Column::DataType,
                            data_account::Column::SlotUpdated,
                        ])
                        .to_owned(),
                )
                .build(DbBackend::Postgres);

            query.sql = format!(
                "{} WHERE excluded.slot_updated >= data_account.slot_updated OR data_account.slot_updated IS NULL",
                query.sql);

            let txn = db.begin().await?;
            txn.execute(query)
                .await
                .map_err(|db_err| IndexerError::AssetIndexError(db_err.to_string()))?;
            txn.commit().await?;
            Ok(())
        }
        _ => Err(IndexerError::NotImplemented),
    }?;
    Ok(())
}
