use enum_iterator::all;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{ConnectionTrait, DatabaseBackend, Statement},
};

use crate::model::table::{PolicyAccount, PolicyAccountType, PolicyEngine, PolicyEngineVersion};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PolicyEngine::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PolicyEngine::Id)
                            .binary()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PolicyEngine::AssetMint).binary().not_null())
                    .col(ColumnDef::new(PolicyEngine::Authority).binary().not_null())
                    .col(ColumnDef::new(PolicyEngine::Delegate).binary().not_null())
                    .col(
                        ColumnDef::new(PolicyEngine::MaxTimeframe)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PolicyEngine::Version)
                            .enumeration(
                                PolicyEngine::PolicyEngineVersion,
                                all::<PolicyEngineVersion>().collect::<Vec<_>>(),
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(PolicyEngine::Policies).json_binary())
                    .col(ColumnDef::new(PolicyEngine::Closed).boolean().not_null())
                    .col(
                        ColumnDef::new(PolicyEngine::SlotUpdated)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PolicyEngine::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(PolicyEngine::LastUpdatedAt)
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
                    .table(PolicyAccount::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PolicyAccount::Id)
                            .binary()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PolicyAccount::PolicyEngine)
                            .binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PolicyAccount::ComparsionType)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PolicyAccount::IdentityLevels).json_binary())
                    .col(ColumnDef::new(PolicyAccount::Timeframe).big_integer())
                    .col(
                        ColumnDef::new(PolicyAccount::PolicyType)
                            .enumeration(
                                PolicyAccount::PolicyAccountType,
                                all::<PolicyAccountType>().collect::<Vec<_>>(),
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PolicyAccount::SlotUpdated)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PolicyAccount::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(PolicyAccount::LastUpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                DatabaseBackend::Postgres,
                r#"ALTER TABLE policy_account ADD COLUMN total_limit "uint64_t";"#.to_string(),
            ))
            .await?;

        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PolicyAccount::Table).to_owned())
            .await?;

        Ok(())
    }
}
