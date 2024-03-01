use sea_orm_migration::prelude::*;

use crate::model::table::{
    ApprovalAccount, AssetController, DataAccount, DataRegistry, IdentityAccount, IdentityApproval,
    IdentityRegistry, PolicyEngineAccount, TrackerAccount, TransactionAmountLimit,
    TransactionAmountVelocity, TransactionCountVelocity,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name("idx_approval_account_mint")
                    .col(ApprovalAccount::AssetMint)
                    .table(ApprovalAccount::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_asset_controller_mint")
                    .col(AssetController::AssetMint)
                    .table(AssetController::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tracker_account_mint")
                    .col(TrackerAccount::AssetMint)
                    .table(TrackerAccount::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_data_registry_mint")
                    .col(DataRegistry::AssetMint)
                    .table(DataRegistry::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_identity_registry_mint")
                    .col(IdentityRegistry::AssetMint)
                    .table(IdentityRegistry::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_policy_engine_mint")
                    .col(PolicyEngineAccount::AssetMint)
                    .table(PolicyEngineAccount::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_data_account_registry")
                    .col(DataAccount::DataRegistry)
                    .table(DataAccount::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_identity_account_registry")
                    .col(IdentityAccount::IdentityRegistry)
                    .table(IdentityAccount::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_identity_approval_policy_engine")
                    .col(IdentityApproval::PolicyEngine)
                    .table(IdentityApproval::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_transaction_amount_limit_policy_engine")
                    .col(TransactionAmountLimit::PolicyEngine)
                    .table(TransactionAmountLimit::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_transaction_amount_velocity_policy_engine")
                    .col(TransactionAmountVelocity::PolicyEngine)
                    .table(TransactionAmountVelocity::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_transaction_count_velocity_policy_engine")
                    .col(TransactionCountVelocity::PolicyEngine)
                    .table(TransactionCountVelocity::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_approval_account_mint")
                    .table(ApprovalAccount::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_asset_controller_mint")
                    .table(AssetController::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_tracker_account_mint")
                    .table(TrackerAccount::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_data_registry_mint")
                    .table(DataRegistry::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_identity_registry_mint")
                    .table(IdentityRegistry::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_policy_engine_mint")
                    .table(PolicyEngineAccount::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_data_account_registry")
                    .table(DataAccount::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_identity_account_registry")
                    .table(IdentityAccount::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_identity_approval_policy_engine")
                    .table(IdentityApproval::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_transaction_amount_limit_policy_engine")
                    .table(TransactionAmountLimit::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_transaction_amount_velocity_policy_engine")
                    .table(TransactionAmountVelocity::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_transaction_count_velocity_policy_engine")
                    .table(TransactionCountVelocity::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
