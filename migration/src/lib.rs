pub use sea_orm_migration::prelude::*;

mod m20220101_000001_init;
mod m20240226_180047_create_asset_controller;
mod m20240226_180544_create_data_registry;
mod m20240226_180606_create_identity_registry;
mod m20240226_180630_create_policy_engine;
mod m20240301_101641_create_inital_indices;
mod model;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_init::Migration),
            Box::new(m20240226_180047_create_asset_controller::Migration),
            Box::new(m20240226_180544_create_data_registry::Migration),
            Box::new(m20240226_180606_create_identity_registry::Migration),
            Box::new(m20240226_180630_create_policy_engine::Migration),
            Box::new(m20240301_101641_create_inital_indices::Migration),
        ]
    }
}
