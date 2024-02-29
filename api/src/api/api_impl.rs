use std::str::FromStr;

use open_rpc_derive::document_rpc;
use open_rpc_schema::document::OpenrpcDocument;
use rwa_types::rapi::{get_all_accounts, FullAccount};
use sea_orm::{ConnectionTrait, DbBackend, Statement};
use solana_sdk::pubkey::Pubkey;
use sqlx::postgres::PgPoolOptions;
use {
    crate::api::*,
    crate::config::Config,
    crate::error::RwaApiError,
    async_trait::async_trait,
    sea_orm::{DatabaseConnection, SqlxPostgresConnector},
};

pub struct RwaApi {
    db_connection: DatabaseConnection,
}

pub fn validate_pubkey(str_pubkey: String) -> Result<Pubkey, RwaApiError> {
    Pubkey::from_str(&str_pubkey).map_err(|_| RwaApiError::PubkeyValidationError(str_pubkey))
}

impl RwaApi {
    pub async fn from_config(config: Config) -> Result<Self, RwaApiError> {
        let pool = PgPoolOptions::new()
            .max_connections(250)
            .connect(&config.database_url)
            .await?;

        let conn = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
        Ok(RwaApi {
            db_connection: conn,
        })
    }
}

#[document_rpc]
#[async_trait]
impl ApiContract for RwaApi {
    // Liveness probe determines if the pod is healthy. Kubernetes will restart the pod if this fails.
    async fn liveness(self: &RwaApi) -> Result<(), RwaApiError> {
        Ok(())
    }

    // Readiness probe determines if the pod has capacity to accept traffic. Kubernetes will not route traffic to this pod if this fails.
    // We are essentially checking if there are DB connections available.
    async fn readiness(self: &RwaApi) -> Result<(), RwaApiError> {
        self.db_connection
            .execute(Statement::from_string(
                DbBackend::Postgres,
                "SELECT 1".to_string(),
            ))
            .await?;
        Ok(())
    }

    async fn get_all_accounts(
        self: &RwaApi,
        payload: GetAllAccounts,
    ) -> Result<FullAccount, RwaApiError> {
        let GetAllAccounts { id } = payload;
        let id_bytes = validate_pubkey(id.clone())?.to_bytes().to_vec();

        get_all_accounts(&self.db_connection, id_bytes)
            .await
            .map_err(Into::into)
    }
}
