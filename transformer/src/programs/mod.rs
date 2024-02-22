use asset_controller::AssetControllerProgram;

pub mod asset_controller;

pub enum ProgramParseResult<'a> {
    AssetControllerProgram(&'a AssetControllerProgram),
    Unknown,
}
