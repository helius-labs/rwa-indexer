use enum_iterator::all;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{ConnectionTrait, DatabaseBackend, Statement},
};

use crate::model::table::{AssetController, AssetControllerVersion, TrackerAccount};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AssetController::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AssetController::Id)
                            .binary()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AssetController::AssetMint)
                            .binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AssetController::Authority)
                            .binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AssetController::Delegate)
                            .binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AssetController::Version)
                            .enumeration(
                                AssetController::AssetControllerVersion,
                                all::<AssetControllerVersion>().collect::<Vec<_>>(),
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(AssetController::Closed).boolean().not_null())
                    .col(
                        ColumnDef::new(AssetController::SlotUpdated)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AssetController::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(AssetController::LastUpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TrackerAccount::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TrackerAccount::Id)
                            .binary()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TrackerAccount::AssetMint)
                            .binary()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TrackerAccount::Owner).binary().not_null())
                    .col(ColumnDef::new(TrackerAccount::TransferAmounts).json_binary())
                    .col(ColumnDef::new(TrackerAccount::TransferTimestamps).json_binary())
                    .col(
                        ColumnDef::new(TrackerAccount::SlotUpdated)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TrackerAccount::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TrackerAccount::LastUpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AssetController::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(TrackerAccount::Table).to_owned())
            .await?;
        Ok(())
    }
}
