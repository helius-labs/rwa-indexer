use crate::error::RwaApiError;
use async_trait::async_trait;
use open_rpc_derive::{document_rpc, rpc};
use open_rpc_schema::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod api_impl;
pub use api_impl::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GetRwaAccountsByMint {
    pub id: String,
}

#[document_rpc]
#[async_trait]
pub trait ApiContract: Send + Sync + 'static {
    async fn liveness(&self) -> Result<(), RwaApiError>;
    async fn readiness(&self) -> Result<(), RwaApiError>;

    #[rpc(
        name = "getRwaAccountsByMint",
        params = "named",
        summary = "Get all RWA accounts by its mint"
    )]
    async fn get_rwa_accounts_by_mint(
        &self,
        payload: GetRwaAccountsByMint,
    ) -> Result<rwa_types::rapi::FullAccount, RwaApiError>;
}
