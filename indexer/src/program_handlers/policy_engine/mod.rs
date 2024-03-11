use crate::{config::IndexerConfig, error::IndexerError};
use num_traits::FromPrimitive;
use plerkle_serialization::AccountInfo;
use policy_engine::Policy;
use rwa_types::dao::{
    policy_account, policy_engine as engine,
    sea_orm_active_enums::{PolicyAccountType, PolicyEngineVersion},
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
            let active_model = engine::ActiveModel {
                id: Set(key_bytes.clone()),
                asset_mint: Set(pe.asset_mint.to_bytes().to_vec()),
                authority: Set(pe.authority.to_bytes().to_vec()),
                delegate: Set(pe.delegate.to_bytes().to_vec()),
                max_timeframe: Set(pe.max_timeframe),
                policies: Set(Some(json!({ "policies": pe.policies}))),
                version: Set(PolicyEngineVersion::from(pe.version)),
                closed: Set(false),
                slot_updated: Set(account_update.slot() as i64),
                ..Default::default()
            };

            let mut query = engine::Entity::insert(active_model)
                .on_conflict(
                    OnConflict::columns([engine::Column::Id])
                        .update_columns([
                            engine::Column::AssetMint,
                            engine::Column::Authority,
                            engine::Column::Delegate,
                            engine::Column::MaxTimeframe,
                            engine::Column::Policies,
                            engine::Column::Version,
                            engine::Column::SlotUpdated,
                        ])
                        .to_owned(),
                )
                .build(DbBackend::Postgres);

            query.sql = format!(
                "{} WHERE excluded.slot_updated >= policy_engine.slot_updated OR policy_engine.slot_updated IS NULL",
                query.sql);

            let txn = db.begin().await?;
            txn.execute(query)
                .await
                .map_err(|db_err| IndexerError::AssetIndexError(db_err.to_string()))?;
            txn.commit().await?;
            Ok(())
        }
        PolicyEngineProgram::PolicyAccount(pe) => {
            let (limit, timeframe) = match pe.policy {
                Policy::TransactionAmountLimit { limit } => (Some(limit), None),
                Policy::TransactionAmountVelocity { limit, timeframe } => {
                    (Some(limit), Some(timeframe))
                }
                Policy::TransactionCountVelocity { limit, timeframe } => {
                    (Some(limit), Some(timeframe))
                }
                _ => (None, None),
            };

            let total_limit: Option<sqlx::types::Decimal> = limit
                .map(|l| sqlx::types::Decimal::from_u64(l).expect("Failed to convert to Decimal"));

            let active_model = policy_account::ActiveModel {
                id: Set(key_bytes.clone()),
                policy_type: Set(PolicyAccountType::from(pe.policy.clone())),
                policy_engine: Set(pe.policy_engine.to_bytes().to_vec()),
                comparsion_type: Set(pe.identity_filter.comparision_type.clone() as i32),
                identity_levels: Set(Some(
                    json!({ "identity_levels": pe.identity_filter.identity_levels}),
                )),
                slot_updated: Set(account_update.slot() as i64),
                timeframe: Set(timeframe),
                total_limit: Set(total_limit),
                ..Default::default()
            };

            let mut query = policy_account::Entity::insert(active_model)
                .on_conflict(
                    OnConflict::columns([policy_account::Column::Id])
                        .update_columns([
                            policy_account::Column::PolicyType,
                            policy_account::Column::ComparsionType,
                            policy_account::Column::IdentityLevels,
                            policy_account::Column::PolicyEngine,
                            policy_account::Column::Timeframe,
                            policy_account::Column::TotalLimit,
                            policy_account::Column::SlotUpdated,
                        ])
                        .to_owned(),
                )
                .build(DbBackend::Postgres);

            query.sql = format!(
                "{} WHERE excluded.slot_updated >= policy_account.slot_updated OR policy_account.slot_updated IS NULL",
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
