use asset_controller::AssetControllerProgram;
use data_registry::DataRegistryProgram;
use identity_registry::IdentityRegistryProgram;
use policy_engine::PolicyEngineProgram;

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
