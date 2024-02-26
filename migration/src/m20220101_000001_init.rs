use sea_orm_migration::{
    prelude::*,
    sea_orm::{ConnectionTrait, DatabaseBackend, Statement},
    sea_query::extension::postgres::Type,
};

use crate::model::table::{
    AssetController, AssetControllerVersion, DataAccount, DataAccountType, DataRegistry,
    DataRegistryVersion, IdentityAccount, IdentityAccountVersion, IdentityApproval,
    IdentityApprovalVersion, IdentityRegistry, IdentityRegistryVersion, PolicyEngineAccount,
    PolicyEngineAccountVersion, TransactionAmountLimit, TransactionAmountLimitVersion,
    TransactionAmountVelocity, TransactionAmountVelocityVersion, TransactionCountVelocity,
    TransactionCountVelocityVersion,
};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                DatabaseBackend::Postgres,
                "
            DO $$
            DECLARE
                type_exists BOOLEAN := EXISTS (SELECT 1 FROM pg_type WHERE typname = 'uint64_t');
            BEGIN
                IF NOT type_exists THEN
                    CREATE DOMAIN uint64_t AS numeric(20, 0) CHECK (VALUE >= 0);
                END IF;
            END $$;
            "
                .to_string(),
            ))
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(AssetController::AssetControllerVersion)
                    .values(vec![AssetControllerVersion::V1])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(DataRegistry::DataRegistryVersion)
                    .values(vec![DataRegistryVersion::V1])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(DataAccount::DataAccountType)
                    .values(vec![
                        DataAccountType::Title,
                        DataAccountType::Legal,
                        DataAccountType::Tax,
                        DataAccountType::Miscellaneous,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(IdentityAccount::IdentityAccountVersion)
                    .values(vec![IdentityAccountVersion::V1])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(IdentityRegistry::IdentityRegistryVersion)
                    .values(vec![IdentityRegistryVersion::V1])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(PolicyEngineAccount::PolicyEngineAccountVersion)
                    .values(vec![PolicyEngineAccountVersion::V1])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(IdentityApproval::IdentityApprovalVersion)
                    .values(vec![IdentityApprovalVersion::V1])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(TransactionAmountVelocity::TransactionAmountVelocityVersion)
                    .values(vec![TransactionAmountVelocityVersion::V1])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(TransactionAmountLimit::TransactionAmountLimitVersion)
                    .values(vec![TransactionAmountLimitVersion::V1])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(TransactionCountVelocity::TransactionCountVelocityVersion)
                    .values(vec![TransactionCountVelocityVersion::V1])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
