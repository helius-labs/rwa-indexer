use crate::{config::IndexerConfig, error::IndexerError};
use plerkle_serialization::AccountInfo;
use rwa_types::dao::{
    policy_account,
    sea_orm_active_enums::{PolicyAccountType, PolicyAccountVersion},
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
        PolicyEngineProgram::PolicyAccount(pe) => {
            let active_model = policy_account::ActiveModel {
                id: Set(key_bytes.clone()),
                //asset_mint: Set(pe.asset_mint.to_bytes().to_vec()),
                policy_type: Set(PolicyAccountType::from(pe.policy.clone())),
                version: Set(PolicyAccountVersion::from(pe.version)),
                comparsion_type: Set(pe.identity_filter.comparision_type.clone() as i32),
                identity_levels: Set(Some(
                    json!({ "identity_levels": pe.identity_filter.identity_levels}),
                )),
                slot_updated: Set(account_update.slot() as i64),
                ..Default::default()
            };

            let mut query = policy_account::Entity::insert(active_model)
                .on_conflict(
                    OnConflict::columns([policy_account::Column::Id])
                        .update_columns([
                            policy_account::Column::AssetMint,
                            policy_account::Column::PolicyType,
                            policy_account::Column::ComparsionType,
                            policy_account::Column::IdentityLevels,
                            policy_account::Column::Version,
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
