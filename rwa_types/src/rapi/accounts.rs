use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::dao::{asset_controller, data_registry, identity_registry, policy_engine};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AssetControllerAccount {
    pub address: String,
    pub mint: String,
    pub authority: String,
    pub delegate: String,
    pub version: u8,
    pub closed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DataRegistryAccount {
    pub address: String,
    pub mint: String,
    pub version: u8,
    pub closed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IdentityRegistryAccount {
    pub address: String,
    pub mint: String,
    pub authority: String,
    pub delegate: String,
    pub version: u8,
    pub closed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PolicyEngine {
    pub address: String,
    pub mint: String,
    pub authority: String,
    pub delegate: String,
    pub policies: Vec<String>,
    pub version: u8,
    pub closed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FullAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_controller: Option<AssetControllerAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_registry: Option<DataRegistryAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_registry: Option<IdentityRegistryAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_engine: Option<PolicyEngine>,
}

impl From<asset_controller::Model> for AssetControllerAccount {
    fn from(asset: asset_controller::Model) -> Self {
        AssetControllerAccount {
            address: bs58::encode(asset.clone().id).into_string(),
            mint: bs58::encode(asset.asset_mint).into_string(),
            authority: bs58::encode(asset.authority).into_string(),
            delegate: bs58::encode(asset.delegate).into_string(),
            version: asset.version as u8,
            closed: asset.closed,
        }
    }
}

impl From<data_registry::Model> for DataRegistryAccount {
    fn from(data: data_registry::Model) -> Self {
        DataRegistryAccount {
            address: bs58::encode(data.clone().id).into_string(),
            mint: bs58::encode(data.asset_mint).into_string(),
            version: data.version as u8,
            closed: data.closed,
        }
    }
}

impl From<identity_registry::Model> for IdentityRegistryAccount {
    fn from(identity: identity_registry::Model) -> Self {
        IdentityRegistryAccount {
            address: bs58::encode(identity.clone().id).into_string(),
            mint: bs58::encode(identity.asset_mint).into_string(),
            authority: bs58::encode(identity.authority).into_string(),
            delegate: bs58::encode(identity.delegate).into_string(),
            version: identity.version as u8,
            closed: identity.closed,
        }
    }
}

impl From<policy_engine::Model> for PolicyEngine {
    fn from(policy: policy_engine::Model) -> Self {
        let policies: Vec<String> = policy.policies.clone().map_or(Vec::new(), |json_value| {
            if let sea_orm::JsonValue::Object(mut obj) = json_value {
                if let Some(sea_orm::JsonValue::Array(arr)) = obj.remove("policies") {
                    arr.into_iter()
                        .map(|item| {
                            if let sea_orm::JsonValue::Array(numbers) = item {
                                let pubkey_bytes: Vec<u8> = numbers
                                    .into_iter()
                                    .filter_map(|n| n.as_u64().map(|num| num as u8))
                                    .collect();
                                bs58::encode(pubkey_bytes).into_string()
                            } else {
                                String::new()
                            }
                        })
                        .collect()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        });

        PolicyEngine {
            address: bs58::encode(policy.clone().id).into_string(),
            mint: bs58::encode(policy.asset_mint).into_string(),
            authority: bs58::encode(policy.authority).into_string(),
            delegate: bs58::encode(policy.delegate).into_string(),
            policies,
            version: policy.version as u8,
            closed: policy.closed,
        }
    }
}
