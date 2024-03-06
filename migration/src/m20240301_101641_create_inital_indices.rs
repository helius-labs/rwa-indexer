use sea_orm_migration::prelude::*;

use crate::model::table::{
    AssetController, DataAccount, DataRegistry, IdentityAccount, IdentityRegistry, PolicyAccount,
    TrackerAccount,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
                    .col(PolicyAccount::AssetMint)
                    .table(PolicyAccount::Table)
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

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
                    .table(PolicyAccount::Table)
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

        Ok(())
    }
}
