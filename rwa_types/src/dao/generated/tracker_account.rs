//! SeaORM Entity. Generated by sea-orm-codegen 0.9.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "tracker_account"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: Vec<u8>,
    pub asset_mint: Vec<u8>,
    pub owner: Vec<u8>,
    pub transfer_amounts: Option<Json>,
    pub transfer_timestamps: Option<Json>,
    pub slot_updated: i64,
    pub created_at: DateTime,
    pub last_updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    AssetMint,
    Owner,
    TransferAmounts,
    TransferTimestamps,
    SlotUpdated,
    CreatedAt,
    LastUpdatedAt,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = Vec<u8>;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Binary.def(),
            Self::AssetMint => ColumnType::Binary.def(),
            Self::Owner => ColumnType::Binary.def(),
            Self::TransferAmounts => ColumnType::JsonBinary.def().null(),
            Self::TransferTimestamps => ColumnType::JsonBinary.def().null(),
            Self::SlotUpdated => ColumnType::BigInteger.def(),
            Self::CreatedAt => ColumnType::DateTime.def(),
            Self::LastUpdatedAt => ColumnType::DateTime.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
