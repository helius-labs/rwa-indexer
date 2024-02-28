use crate::{config::IndexerConfig, error::IndexerError};
use plerkle_serialization::AccountInfo;
use rwa_types::dao::{
    identity_approval, policy_engine_account,
    sea_orm_active_enums::{
        IdentityApprovalVersion, PolicyEngineAccountVersion, TransactionAmountLimitVersion,
        TransactionAmountVelocityVersion, TransactionCountVelocityVersion,
    },
    transaction_amount_limit, transaction_amount_velocity, transaction_count_velocity,
};
use sea_orm::{
    query::*, sea_query::OnConflict, ActiveValue::Set, ConnectionTrait, DatabaseConnection,
    DbBackend, EntityTrait,
};
use serde_json::json;
use transformer::programs::policy_engine::PolicyEngineProgram;

pub async fn handle_policy_engine_program_account<'a, 'b, 'c>(
    account_update: &'a AccountInfo<'a>,
    parsing_result: &'b PolicyEngineProgram,
    db: &'c DatabaseConnection,
    _config: &IndexerConfig,
) -> Result<(), IndexerError> {
    let key = *account_update.pubkey().unwrap();
    let key_bytes = key.0.to_vec();
    match &parsing_result {
        PolicyEngineProgram::PolicyEngine(pe) => {
            let active_model = policy_engine_account::ActiveModel {
                id: Set(key_bytes.clone()),
                asset_mint: Set(pe.asset_mint.to_bytes().to_vec()),
                authority: Set(pe.authority.to_bytes().to_vec()),
                delegate: Set(pe.delegate.to_bytes().to_vec()),
                max_time_frame: Set(pe.max_timeframe),
                version: Set(PolicyEngineAccountVersion::from(pe.version)),
                slot_updated: Set(account_update.slot() as i64),
                ..Default::default()
            };

            let mut query = policy_engine_account::Entity::insert(active_model)
                .on_conflict(
                    OnConflict::columns([policy_engine_account::Column::Id])
                        .update_columns([
                            policy_engine_account::Column::AssetMint,
                            policy_engine_account::Column::Authority,
                            policy_engine_account::Column::Delegate,
                            policy_engine_account::Column::MaxTimeFrame,
                            policy_engine_account::Column::Version,
                            policy_engine_account::Column::SlotUpdated,
                        ])
                        .to_owned(),
                )
                .build(DbBackend::Postgres);

            query.sql = format!(
                "{} WHERE excluded.slot_updated >= policy_engine_account.slot_updated OR policy_engine_account.slot_updated IS NULL",
                query.sql);

            let txn = db.begin().await?;
            txn.execute(query)
                .await
                .map_err(|db_err| IndexerError::AssetIndexError(db_err.to_string()))?;
            txn.commit().await?;
            Ok(())
        }
        PolicyEngineProgram::IdentityApproval(ia) => {
            let active_model = identity_approval::ActiveModel {
                id: Set(key_bytes.clone()),
                comparsion_type: Set(ia.identity_filter.comparision_type as i32),
                identity_levels: Set(Some(
                    json!({ "identity_levels": ia.identity_filter.identity_levels}),
                )),
                version: Set(IdentityApprovalVersion::from(ia.version)),
                slot_updated: Set(account_update.slot() as i64),
                ..Default::default()
            };

            let mut query = identity_approval::Entity::insert(active_model)
                .on_conflict(
                    OnConflict::columns([identity_approval::Column::Id])
                        .update_columns([
                            identity_approval::Column::ComparsionType,
                            identity_approval::Column::Version,
                            identity_approval::Column::SlotUpdated,
                        ])
                        .to_owned(),
                )
                .build(DbBackend::Postgres);

            query.sql = format!(
                "{} WHERE excluded.slot_updated >= identity_approval.slot_updated OR identity_approval.slot_updated IS NULL",
                query.sql);

            let txn = db.begin().await?;
            txn.execute(query)
                .await
                .map_err(|db_err| IndexerError::AssetIndexError(db_err.to_string()))?;
            txn.commit().await?;
            Ok(())
        }
        PolicyEngineProgram::TransactionAmountLimit(ta) => {
            let active_model = transaction_amount_limit::ActiveModel {
                id: Set(key_bytes.clone()),
                comparsion_type: Set(ta.identity_filter.comparision_type as i32),
                identity_levels: Set(Some(
                    json!({ "identity_levels": ta.identity_filter.identity_levels}),
                )),
                version: Set(TransactionAmountLimitVersion::from(ta.version)),
                slot_updated: Set(account_update.slot() as i64),
                ..Default::default()
            };

            let mut query = transaction_amount_limit::Entity::insert(active_model)
                .on_conflict(
                    OnConflict::columns([transaction_amount_limit::Column::Id])
                        .update_columns([
                            transaction_amount_limit::Column::ComparsionType,
                            transaction_amount_limit::Column::Version,
                            transaction_amount_limit::Column::SlotUpdated,
                        ])
                        .to_owned(),
                )
                .build(DbBackend::Postgres);

            query.sql = format!(
                "{} WHERE excluded.slot_updated >= transaction_amount_limit.slot_updated OR transaction_amount_limit.slot_updated IS NULL",
                query.sql);

            let txn = db.begin().await?;
            txn.execute(query)
                .await
                .map_err(|db_err| IndexerError::AssetIndexError(db_err.to_string()))?;
            txn.commit().await?;
            Ok(())
        }
        PolicyEngineProgram::TransactionAmountVelocity(tv) => {
            let active_model = transaction_amount_velocity::ActiveModel {
                id: Set(key_bytes.clone()),
                comparsion_type: Set(tv.identity_filter.comparision_type as i32),
                identity_levels: Set(Some(
                    json!({ "identity_levels": tv.identity_filter.identity_levels }),
                )),
                version: Set(TransactionAmountVelocityVersion::from(tv.version)),
                slot_updated: Set(account_update.slot() as i64),
                ..Default::default()
            };

            let mut query = transaction_amount_velocity::Entity::insert(active_model)
                .on_conflict(
                    OnConflict::columns([transaction_amount_velocity::Column::Id])
                        .update_columns([
                            transaction_amount_velocity::Column::ComparsionType,
                            transaction_amount_velocity::Column::Version,
                            transaction_amount_velocity::Column::SlotUpdated,
                        ])
                        .to_owned(),
                )
                .build(DbBackend::Postgres);
            query.sql = format!(
                "{} WHERE excluded.slot_updated >= transaction_amount_velocity.slot_updated OR transaction_amount_velocity.slot_updated IS NULL",
                query.sql);

            let txn = db.begin().await?;
            txn.execute(query)
                .await
                .map_err(|db_err| IndexerError::AssetIndexError(db_err.to_string()))?;
            txn.commit().await?;

            Ok(())
        }
        PolicyEngineProgram::TransactionCountVelocity(tc) => {
            let active_model = transaction_count_velocity::ActiveModel {
                id: Set(key_bytes.clone()),
                comparsion_type: Set(tc.identity_filter.comparision_type as i32),
                identity_levels: Set(Some(
                    json!({ "identity_levels": tc.identity_filter.identity_levels}),
                )),
                version: Set(TransactionCountVelocityVersion::from(tc.version)),
                slot_updated: Set(account_update.slot() as i64),
                ..Default::default()
            };

            let mut query = transaction_count_velocity::Entity::insert(active_model)
                .on_conflict(
                    OnConflict::columns([transaction_count_velocity::Column::Id])
                        .update_columns([
                            transaction_count_velocity::Column::ComparsionType,
                            transaction_count_velocity::Column::Version,
                            transaction_count_velocity::Column::SlotUpdated,
                        ])
                        .to_owned(),
                )
                .build(DbBackend::Postgres);
            query.sql = format!(
                "{} WHERE excluded.slot_updated >= transaction_count_velocity.slot_updated OR transaction_count_velocity.slot_updated IS NULL",
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
