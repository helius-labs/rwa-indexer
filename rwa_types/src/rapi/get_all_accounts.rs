use crate::dao::{asset_controller, data_registry, identity_registry, policy_engine_account};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use super::{
    AssetControllerAccount, DataRegistryAccount, FullAccount, IdentityRegistryAccount,
    PolicyEngineAccount,
};

pub async fn get_all_accounts_controller(
    db: &DatabaseConnection,
    id: Vec<u8>,
) -> Result<Option<asset_controller::Model>, DbErr> {
    let account = asset_controller::Entity::find()
        .filter(asset_controller::Column::AssetMint.eq(id.clone()))
        .one(db)
        .await?;
    Ok(account)
}

pub async fn get_data_registry(
    db: &DatabaseConnection,
    id: Vec<u8>,
) -> Result<Option<data_registry::Model>, DbErr> {
    let account = data_registry::Entity::find()
        .filter(data_registry::Column::AssetMint.eq(id.clone()))
        .one(db)
        .await?;
    Ok(account)
}

pub async fn get_identity_registry(
    db: &DatabaseConnection,
    id: Vec<u8>,
) -> Result<Option<identity_registry::Model>, DbErr> {
    let account = identity_registry::Entity::find()
        .filter(identity_registry::Column::AssetMint.eq(id.clone()))
        .one(db)
        .await?;
    Ok(account)
}

pub async fn get_policy_engine(
    db: &DatabaseConnection,
    id: Vec<u8>,
) -> Result<Option<policy_engine_account::Model>, DbErr> {
    let account = policy_engine_account::Entity::find()
        .filter(policy_engine_account::Column::AssetMint.eq(id.clone()))
        .one(db)
        .await?;
    Ok(account)
}

pub async fn get_all_accounts(db: &DatabaseConnection, id: Vec<u8>) -> Result<FullAccount, DbErr> {
    let get_accounts_controller_future = get_all_accounts_controller(db, id.clone());
    let get_data_registry_future = get_data_registry(db, id.clone());
    let get_identity_registry_future = get_identity_registry(db, id.clone());
    let get_policy_engine_future = get_policy_engine(db, id.clone());

    let (asset_controller, data_registry, identity_registry, policy_engine) = tokio::join!(
        get_accounts_controller_future,
        get_data_registry_future,
        get_identity_registry_future,
        get_policy_engine_future
    );

    Ok(FullAccount {
        asset_controller: asset_controller
            .ok()
            .and_then(|opt| opt.map(AssetControllerAccount::from)),
        data_registry: data_registry
            .ok()
            .and_then(|opt| opt.map(DataRegistryAccount::from)),
        identity_registry: identity_registry
            .ok()
            .and_then(|opt| opt.map(IdentityRegistryAccount::from)),
        policy_engine: policy_engine
            .ok()
            .and_then(|opt| opt.map(PolicyEngineAccount::from)),
    })
}
