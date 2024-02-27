use crate::{config::IndexerConfig, error::IndexerError};
use plerkle_serialization::AccountInfo;
use rwa_types::dao::{
    approval_account, asset_controller, sea_orm_active_enums::AssetControllerVersion,
    tracker_account,
};
use sea_orm::{
    query::*, sea_query::OnConflict, ActiveValue::Set, ConnectionTrait, DatabaseConnection,
    DbBackend, EntityTrait,
};
use transformer::programs::asset_controller::AssetControllerProgram;

pub async fn handle_asset_controller_program_account<'a, 'b, 'c>(
    account_update: &'a AccountInfo<'a>,
    parsing_result: &'b AssetControllerProgram,
    db: &'c DatabaseConnection,
    _config: &IndexerConfig,
) -> Result<(), IndexerError> {
    let key = *account_update.pubkey().unwrap();
    let key_bytes = key.0.to_vec();
    match &parsing_result {
        AssetControllerProgram::AssetControllerAccount(ac) => {
            let active_model = asset_controller::ActiveModel {
                id: Set(key_bytes.clone()),
                asset_mint: Set(ac.asset_mint.to_bytes().to_vec()),
                authority: Set(ac.authority.to_bytes().to_vec()),
                delegate: Set(ac.delegate.to_bytes().to_vec()),
                version: Set(AssetControllerVersion::from(ac.version)),
                closed: Set(false),
                slot_updated: Set(account_update.slot() as i64),
                ..Default::default()
            };

            let mut query = asset_controller::Entity::insert(active_model)
                .on_conflict(
                    OnConflict::columns([asset_controller::Column::Id])
                        .update_columns([
                            asset_controller::Column::AssetMint,
                            asset_controller::Column::Authority,
                            asset_controller::Column::Delegate,
                            asset_controller::Column::Version,
                            asset_controller::Column::Closed,
                            asset_controller::Column::SlotUpdated,
                        ])
                        .to_owned(),
                )
                .build(DbBackend::Postgres);

            query.sql = format!(
                "{} WHERE excluded.slot_updated >= asset_controller.slot_updated OR asset.slot_updated IS NULL",
                query.sql);

            let txn = db.begin().await?;
            txn.execute(query)
                .await
                .map_err(|db_err| IndexerError::AssetIndexError(db_err.to_string()))?;

            txn.commit().await?;
            Ok(())
        }
        AssetControllerProgram::TransactionApprovalAccount(taa) => {
            let amount = taa.amount.map(sqlx::types::Decimal::from);
            let active_model = approval_account::ActiveModel {
                id: Set(key_bytes.clone()),
                asset_mint: Set(taa.asset_mint.to_bytes().to_vec()),
                from_token_account: Set(taa
                    .from_token_account
                    .map(|account| account.to_bytes().to_vec())),
                to_token_account: Set(taa
                    .to_token_account
                    .map(|account| account.to_bytes().to_vec())),
                expiry_slot: Set(taa.slot as i64),
                amount: Set(amount),
                slot_updated: Set(account_update.slot() as i64),
                ..Default::default()
            };

            let mut query = approval_account::Entity::insert(active_model)
                .on_conflict(
                    OnConflict::columns([approval_account::Column::Id])
                        .update_columns([
                            approval_account::Column::AssetMint,
                            approval_account::Column::FromTokenAccount,
                            approval_account::Column::ToTokenAccount,
                            approval_account::Column::ExpirySlot,
                            approval_account::Column::Amount,
                            approval_account::Column::SlotUpdated,
                        ])
                        .to_owned(),
                )
                .build(DbBackend::Postgres);
            query.sql = format!(
                "{} WHERE excluded.slot_updated >= approval_account.slot_updated OR approval_account.slot_updated IS NULL",
                query.sql);

            let txn = db.begin().await?;
            txn.execute(query)
                .await
                .map_err(|db_err| IndexerError::AssetIndexError(db_err.to_string()))?;
            txn.commit().await?;
            Ok(())
        }
        AssetControllerProgram::TrackerAccount(ta) => {
            let active_model = tracker_account::ActiveModel {
                id: Set(key_bytes.clone()),
                asset_mint: Set(ta.asset_mint.to_bytes().to_vec()),
                owner: Set(ta.owner.to_bytes().to_vec()),
                transfer_amounts: Set(serde_json::to_string(&ta.transfer_amounts)
                    .unwrap_or_else(|_| "[]".to_string())),
                transfer_timestamps: Set(serde_json::to_string(&ta.transfer_timestamps)
                    .unwrap_or_else(|_| "[]".to_string())),
                slot_updated: Set(account_update.slot() as i64),
                ..Default::default()
            };

            let mut query = tracker_account::Entity::insert(active_model)
                .on_conflict(
                    OnConflict::columns([tracker_account::Column::Id])
                        .update_columns([
                            tracker_account::Column::AssetMint,
                            tracker_account::Column::Owner,
                            tracker_account::Column::TransferAmounts,
                            tracker_account::Column::TransferTimestamps,
                            tracker_account::Column::SlotUpdated,
                        ])
                        .to_owned(),
                )
                .build(DbBackend::Postgres);

            query.sql = format!(
                "{} WHERE excluded.slot_updated >= tracker_account.slot_updated OR tracker_account.slot_updated IS NULL",
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
