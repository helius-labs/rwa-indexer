use enum_iterator::all;
use sea_orm_migration::prelude::*;

use crate::model::table::{
    IdentityAccount, IdentityAccountVersion, IdentityRegistry, IdentityRegistryVersion,
};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IdentityRegistry::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IdentityRegistry::Id)
                            .binary()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(IdentityRegistry::AssetMint)
                            .binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IdentityRegistry::Authority)
                            .binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IdentityRegistry::Delegate)
                            .binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IdentityRegistry::Version)
                            .enumeration(
                                IdentityRegistry::IdentityRegistryVersion,
                                all::<IdentityRegistryVersion>().collect::<Vec<_>>(),
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IdentityRegistry::Closed)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IdentityRegistry::SlotUpdated)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IdentityRegistry::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(IdentityRegistry::LastUpdatedAt)
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
                    .table(IdentityAccount::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IdentityAccount::Id)
                            .binary()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IdentityAccount::Owner).binary().not_null())
                    .col(
                        ColumnDef::new(IdentityAccount::IdentityRegistry)
                            .binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IdentityAccount::Version)
                            .enumeration(
                                IdentityAccount::IdentityAccountVersion,
                                all::<IdentityAccountVersion>().collect::<Vec<_>>(),
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(IdentityAccount::Levels).json_binary())
                    .col(
                        ColumnDef::new(IdentityAccount::SlotUpdated)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IdentityAccount::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(IdentityAccount::LastUpdatedAt)
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
            .drop_table(Table::drop().table(IdentityAccount::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(IdentityRegistry::Table).to_owned())
            .await?;
        Ok(())
    }
}
