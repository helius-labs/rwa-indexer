use asset_controller::AssetControllerProgram;
use data_registry::DataRegistryProgram;
use identity_registry::IdentityRegistryProgram;
use policy_engine::PolicyEngineProgram;
use solana_sdk::hash::hash;

pub mod asset_controller;
pub mod data_registry;
pub mod identity_registry;
pub mod policy_engine;

pub enum ProgramParseResult<'a> {
    AssetControllerProgram(&'a AssetControllerProgram),
    DataRegistryProgram(&'a DataRegistryProgram),
    IdentityRegistryProgram(&'a IdentityRegistryProgram),
    PolicyEngineProgram(&'a PolicyEngineProgram),
    Unknown,
}

fn get_discriminator(account_type: &str) -> [u8; 8] {
    let discriminator_preimage = format!("account:{}", account_type);
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&hash(discriminator_preimage.as_bytes()).to_bytes()[..8]);
    discriminator
}
