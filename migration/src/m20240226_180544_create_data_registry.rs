use enum_iterator::all;
use sea_orm_migration::prelude::*;

use crate::model::table::{DataAccount, DataAccountType, DataRegistry, DataRegistryVersion};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DataRegistry::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DataRegistry::Id)
                            .binary()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DataRegistry::AssetMint).binary().not_null())
                    .col(ColumnDef::new(DataRegistry::Authority).binary().not_null())
                    .col(
                        ColumnDef::new(DataRegistry::Version)
                            .enumeration(
                                DataRegistry::DataRegistryVersion,
                                all::<DataRegistryVersion>().collect::<Vec<_>>(),
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(DataRegistry::Closed).boolean().not_null())
                    .col(
                        ColumnDef::new(DataRegistry::SlotUpdated)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DataRegistry::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(DataRegistry::LastUpdatedAt)
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
                    .table(DataAccount::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DataAccount::Id)
                            .binary()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DataAccount::Name).string().not_null())
                    .col(ColumnDef::new(DataAccount::Uri).string().not_null())
                    .col(
                        ColumnDef::new(DataAccount::DataRegistry)
                            .binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DataAccount::DataType)
                            .enumeration(
                                DataAccount::DataAccountType,
                                all::<DataAccountType>().collect::<Vec<_>>(),
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DataAccount::SlotUpdated)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DataAccount::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(DataAccount::LastUpdatedAt)
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
            .drop_table(Table::drop().table(DataAccount::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(DataRegistry::Table).to_owned())
            .await?;
        Ok(())
    }
}
