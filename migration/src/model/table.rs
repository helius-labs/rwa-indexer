use enum_iterator::Sequence;
use sea_orm_migration::prelude::*;

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum AssetControllerVersion {
    V1 = 1,
}

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum DataRegistryVersion {
    V1 = 1,
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
    V1 = 1,
}

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum IdentityRegistryVersion {
    V1 = 1,
}

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum PolicyEngineAccountVersion {
    V1 = 1,
}

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum IdentityApprovalVersion {
    V1 = 1,
}

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum TransactionAmountVelocityVersion {
    V1 = 1,
}

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum TransactionAmountLimitVersion {
    V1 = 1,
}

#[derive(Iden, Debug, PartialEq, Sequence)]
pub enum TransactionCountVelocityVersion {
    V1 = 1,
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
pub enum ApprovalAccount {
    #[iden = "approval_account"]
    Table,
    Id,
    AssetMint,
    FromTokenAccount,
    ToTokenAccount,
    Amount,
    ExpirySlot,
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
pub enum PolicyEngineAccount {
    PolicyEngineAccountVersion,
    #[iden = "policy_engine_account"]
    Table,
    Id,
    AssetMint,
    Authority,
    Delegate,
    MaxTimeFrame,
    Polices,
    Version,
    Closed,
    SlotUpdated,
    CreatedAt,
    LastUpdatedAt,
}

#[derive(Copy, Clone, Iden)]
pub enum IdentityApproval {
    IdentityApprovalVersion,
    #[iden = "identity_approval"]
    Table,
    Id,
    PolicyEngine,
    IdentityLevels,
    ComparsionType,
    Version,
    SlotUpdated,
    CreatedAt,
    LastUpdatedAt,
}

#[derive(Copy, Clone, Iden)]
pub enum TransactionAmountVelocity {
    TransactionAmountVelocityVersion,
    #[iden = "transaction_amount_velocity"]
    Table,
    Id,
    PolicyEngine,
    TotalLimit,
    TimeFrame,
    IdentityLevels,
    ComparsionType,
    Version,
    SlotUpdated,
    CreatedAt,
    LastUpdatedAt,
}

#[derive(Copy, Clone, Iden)]
pub enum TransactionAmountLimit {
    TransactionAmountLimitVersion,
    #[iden = "transaction_amount_limit"]
    Table,
    Id,
    PolicyEngine,
    TotalLimit,
    IdentityLevels,
    ComparsionType,
    Version,
    SlotUpdated,
    CreatedAt,
    LastUpdatedAt,
}

#[derive(Copy, Clone, Iden)]
pub enum TransactionCountVelocity {
    TransactionCountVelocityVersion,
    #[iden = "transaction_count_velocity"]
    Table,
    Id,
    PolicyEngine,
    TotalLimit,
    TimeFrame,
    IdentityLevels,
    ComparsionType,
    Version,
    SlotUpdated,
    CreatedAt,
    LastUpdatedAt,
}
