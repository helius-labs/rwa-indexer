use crate::{config::IndexerConfig, error::IndexerError};
use plerkle_serialization::AccountInfo;
use rwa_types::dao::{
    identity_account, identity_registry,
    sea_orm_active_enums::{IdentityAccountVersion, IdentityRegistryVersion},
};
use sea_orm::{
    query::*, sea_query::OnConflict, ActiveValue::Set, ConnectionTrait, DatabaseConnection,
    DbBackend, EntityTrait,
};
use transformer::programs::identity_registry::IdentityRegistryProgram;

pub async fn handle_identity_registry_program_account<'a, 'b, 'c>(
    account_update: &'a AccountInfo<'a>,
    parsing_result: &'b IdentityRegistryProgram,
    db: &'c DatabaseConnection,
    _config: &IndexerConfig,
) -> Result<(), IndexerError> {
    let key = *account_update.pubkey().unwrap();
    let key_bytes = key.0.to_vec();
    match &parsing_result {
        IdentityRegistryProgram::IdentityRegistry(ir) => {
            let active_model = identity_registry::ActiveModel {
                id: Set(key_bytes.clone()),
                asset_mint: Set(ir.asset_mint.to_bytes().to_vec()),
                authority: Set(ir.authority.to_bytes().to_vec()),
                delegate: Set(ir.delegate.to_bytes().to_vec()),
                version: Set(IdentityRegistryVersion::from(ir.version)),
                closed: Set(false),
                slot_updated: Set(account_update.slot() as i64),
                ..Default::default()
            };

            let mut query = identity_registry::Entity::insert(active_model)
                .on_conflict(
                    OnConflict::columns([identity_registry::Column::Id])
                        .update_columns([
                            identity_registry::Column::AssetMint,
                            identity_registry::Column::Authority,
                            identity_registry::Column::Delegate,
                            identity_registry::Column::Version,
                            identity_registry::Column::Closed,
                            identity_registry::Column::SlotUpdated,
                        ])
                        .to_owned(),
                )
                .build(DbBackend::Postgres);

            query.sql = format!(
                "{} WHERE excluded.slot_updated >= identity_registry.slot_updated OR identity_registry.slot_updated IS NULL",
                query.sql);

            let txn = db.begin().await?;
            txn.execute(query)
                .await
                .map_err(|db_err| IndexerError::AssetIndexError(db_err.to_string()))?;
            txn.commit().await?;
            Ok(())
        }
        IdentityRegistryProgram::IdentityAccount(ia) => {
            let active_model = identity_account::ActiveModel {
                id: Set(key_bytes.clone()),
                owner: Set(ia.owner.to_bytes().to_vec()),
                identity_registry: Set(ia.registry.to_bytes().to_vec()),
                version: Set(IdentityAccountVersion::from(ia.version)),
                levels: Set(serde_json::to_string(&ia.levels).unwrap_or_else(|_| "[]".to_string())),
                slot_updated: Set(account_update.slot() as i64),
                ..Default::default()
            };

            let mut query = identity_account::Entity::insert(active_model)
                .on_conflict(
                    OnConflict::columns([identity_account::Column::Id])
                        .update_columns([
                            identity_account::Column::Owner,
                            identity_account::Column::IdentityRegistry,
                            identity_account::Column::Version,
                            identity_account::Column::Levels,
                            identity_account::Column::SlotUpdated,
                        ])
                        .to_owned(),
                )
                .build(DbBackend::Postgres);

            query.sql = format!(
                "{} WHERE excluded.slot_updated >= identity_account.slot_updated OR identity_account.slot_updated IS NULL",
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
