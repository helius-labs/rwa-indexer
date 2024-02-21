use enum_iterator::Sequence;
use sea_orm_migration::prelude::*;

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum AssetControllerVersion {
    V1,
    V2,
}

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum DataRegistryVersion {
    V1,
    V2,
}

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum DataAccountType {
    V1,
    V2,
}

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum IdentityAccountRole {
    V1,
    V2,
}

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum IdentityRegistryVersion {
    V1,
    V2,
}

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum PolicyEngineAccountVersion {
    V1,
    V2,
}

#[derive(Copy, Clone, Iden)]
pub enum AssetController {
    AssetControllerVersion,
    #[iden = "asset_controller"]
    Table,
    Id,
    AssetMint,
    DataUpdateAuthority,
    TransactionApprovalAuthority,
    Version,
    SlotUpdated,
    CreatedAt,
    LastUpdatedAt,
}

#[derive(Copy, Clone, Iden)]
pub enum ApprovalAccount {
    #[iden = "approval_account"]
    Table,
    Id,
    AssetMint,
    ToAddress,
    FromAddress,
    Amount,
    ExpirySlot,
    Closed,
    SlotUpdated,
    CreatedAt,
    LastUpdatedAt,
}

#[derive(Copy, Clone, Iden)]
pub enum DataAccount {
    DataAccountType,
    #[iden = "data_account"]
    Table,
    Id,
    Key,
    Value,
    DataRegistry,
    Type,
    Valid,
    SlotUpdated,
    CreatedAt,
    LastUpdatedAt,
}

#[derive(Copy, Clone, Iden)]
pub enum DataRegistry {
    DataRegistryVersion,
    #[iden = "data_registry"]
    Table,
    Id,
    AssetMint,
    Authority,
    Version,
    SlotUpdated,
    CreatedAt,
    LastUpdatedAt,
}

#[derive(Copy, Clone, Iden)]
pub enum IdentityRegistry {
    IdentityRegistryVersion,
    #[iden = "identity_registry"]
    Table,
    Id,
    AssetMint,
    Authority,
    Version,
    SlotUpdated,
    CreatedAt,
    LastUpdatedAt,
}

#[derive(Copy, Clone, Iden)]
pub enum IdentityAccount {
    IdentityAccountRole,
    #[iden = "identity_account"]
    Table,
    Id,
    Owner,
    Role,
    DataRegistry,
    SlotUpdated,
    Closed,
    CreatedAt,
    LastUpdatedAt,
}

#[derive(Copy, Clone, Iden)]
pub enum PolicyEngineAccount {
    PolicyEngineAccountVersion,
    #[iden = "policy_engine_account"]
    Table,
    Id,
    AssetMint,
    Authority,
    Version,
    SlotUpdated,
    CreatedAt,
    LastUpdatedAt,
}
