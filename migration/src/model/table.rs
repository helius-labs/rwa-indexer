use enum_iterator::Sequence;
use sea_orm_migration::prelude::*;

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum AssetControllerVersion {
    V0,
    V1,
}

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum DataRegistryVersion {
    V0,
    V1,
}

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum DataAccountType {
    Title,
    Legal,
    Tax,
    Miscellaneous,
}

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum IdentityAccountVersion {
    V0,
    V1,
}

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum IdentityRegistryVersion {
    V0,
    V1,
}

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum PolicyEngineVersion {
    V0,
    V1,
}

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum PolicyAccountType {
    IdentityApproval,
    TransactionAmountLimit,
    TransactionAmountVelocity,
    TransactionCountVelocity,
}

#[derive(Copy, Clone, Iden)]
pub enum AssetController {
    AssetControllerVersion,
    #[iden = "asset_controller"]
    Table,
    Id,
    Version,
    AssetMint,
    Authority,
    Delegate,
    Closed,
    SlotUpdated,
    CreatedAt,
    LastUpdatedAt,
}

#[derive(Copy, Clone, Iden)]
pub enum TrackerAccount {
    #[iden = "tracker_account"]
    Table,
    Id,
    AssetMint,
    Owner,
    TransferAmounts,
    TransferTimestamps,
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
    Delegate,
    Closed,
    SlotUpdated,
    CreatedAt,
    LastUpdatedAt,
}

#[derive(Copy, Clone, Iden)]
pub enum DataAccount {
    DataAccountType,
    DataRegistryVersion,
    #[iden = "data_account"]
    Table,
    Id,
    DataRegistry,
    DataType,
    Name,
    Uri,
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
    Delegate,
    Closed,
    SlotUpdated,
    CreatedAt,
    LastUpdatedAt,
}

#[derive(Copy, Clone, Iden)]
pub enum IdentityAccount {
    IdentityAccountVersion,
    #[iden = "identity_account"]
    Table,
    Id,
    Version,
    Owner,
    IdentityRegistry,
    Levels,
    SlotUpdated,
    CreatedAt,
    LastUpdatedAt,
}

#[derive(Copy, Clone, Iden)]
pub enum PolicyEngine {
    PolicyEngineVersion,
    #[iden = "policy_engine"]
    Table,
    Id,
    AssetMint,
    Authority,
    Delegate,
    MaxTimeframe,
    Policies,
    Version,
    Closed,
    SlotUpdated,
    CreatedAt,
    LastUpdatedAt,
}

#[derive(Copy, Clone, Iden)]
pub enum PolicyAccount {
    PolicyAccountType,
    #[iden = "policy_account"]
    Table,
    Id,
    PolicyEngine,
    PolicyType,
    IdentityLevels,
    ComparsionType,
    TotalLimit,
    Timeframe,
    SlotUpdated,
    CreatedAt,
    LastUpdatedAt,
}
