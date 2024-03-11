use sea_orm_migration::{
    prelude::*,
    sea_orm::{ConnectionTrait, DatabaseBackend, Statement},
    sea_query::extension::postgres::Type,
};

use crate::model::table::{
    AssetController, AssetControllerVersion, DataAccount, DataAccountType, DataRegistry,
    DataRegistryVersion, IdentityAccount, IdentityAccountVersion, IdentityRegistry,
    IdentityRegistryVersion, PolicyAccount, PolicyAccountType, PolicyEngine, PolicyEngineVersion,
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
                    .values(vec![AssetControllerVersion::V0, AssetControllerVersion::V1])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(DataRegistry::DataRegistryVersion)
                    .values(vec![DataRegistryVersion::V0, DataRegistryVersion::V1])
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
                    .values(vec![IdentityAccountVersion::V0, IdentityAccountVersion::V1])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(IdentityRegistry::IdentityRegistryVersion)
                    .values(vec![
                        IdentityRegistryVersion::V0,
                        IdentityRegistryVersion::V1,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(PolicyEngine::PolicyEngineVersion)
                    .values(vec![PolicyEngineVersion::V0, PolicyEngineVersion::V1])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(PolicyAccount::PolicyAccountType)
                    .values(vec![
                        PolicyAccountType::IdentityApproval,
                        PolicyAccountType::TransactionAmountLimit,
                        PolicyAccountType::TransactionAmountVelocity,
                        PolicyAccountType::TransactionCountVelocity,
                    ])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
