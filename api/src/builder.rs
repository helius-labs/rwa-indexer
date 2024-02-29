use jsonrpsee::RpcModule;
use log::debug;

use crate::api::*;
use crate::error::RwaApiError;

pub struct RpcApiBuilder;

impl RpcApiBuilder {
    pub fn build(
        contract: Box<dyn ApiContract>,
    ) -> Result<RpcModule<Box<dyn ApiContract>>, RwaApiError> {
        let mut module = RpcModule::new(contract);

        module.register_async_method("liveness", |_rpc_params, rpc_context| async move {
            debug!("Checking Liveness");
            rpc_context.liveness().await.map_err(Into::into)
        })?;

        module.register_async_method("readiness", |_rpc_params, rpc_context| async move {
            debug!("Checking Readiness");
            rpc_context.readiness().await.map_err(Into::into)
        })?;

        // get_all_accounts
        module.register_async_method("get_all_accounts", |rpc_params, rpc_context| async move {
            let payload = rpc_params.parse::<GetAllAccounts>()?;
            rpc_context
                .get_all_accounts(payload)
                .await
                .map_err(Into::into)
        })?;
        module.register_alias("getAllAccounts", "get_all_accounts")?;

        module.register_async_method("schema", |_, rpc_context| async move {
            Ok(rpc_context.schema())
        })?;
        module.register_alias("rwa_schema", "schema")?;
        module.register_alias("rwaSchema", "schema")?;

        Ok(module)
    }
}
