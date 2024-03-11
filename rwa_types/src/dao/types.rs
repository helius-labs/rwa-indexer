use super::sea_orm_active_enums::{
    AssetControllerVersion, DataAccountType, DataRegistryVersion, IdentityAccountVersion,
    IdentityRegistryVersion, PolicyAccountType, PolicyEngineVersion,
};
use data_registry::state::DataAccountType as ProgramDataAccountType;
use policy_engine::Policy;

impl From<u8> for AssetControllerVersion {
    fn from(version: u8) -> Self {
        match version {
            1 => AssetControllerVersion::V1,
            _ => AssetControllerVersion::V1,
        }
    }
}

impl From<u8> for DataRegistryVersion {
    fn from(version: u8) -> Self {
        match version {
            1 => DataRegistryVersion::V1,
            _ => DataRegistryVersion::V1,
        }
    }
}

impl From<u8> for IdentityRegistryVersion {
    fn from(version: u8) -> Self {
        match version {
            1 => IdentityRegistryVersion::V1,
            _ => IdentityRegistryVersion::V1,
        }
    }
}

impl From<u8> for IdentityAccountVersion {
    fn from(version: u8) -> Self {
        match version {
            1 => IdentityAccountVersion::V1,
            _ => IdentityAccountVersion::V1,
        }
    }
}

impl From<ProgramDataAccountType> for DataAccountType {
    fn from(da: ProgramDataAccountType) -> Self {
        match da {
            ProgramDataAccountType::Title => DataAccountType::Title,
            ProgramDataAccountType::Legal => DataAccountType::Legal,
            ProgramDataAccountType::Tax => DataAccountType::Tax,
            ProgramDataAccountType::Miscellaneous => DataAccountType::Miscellaneous,
        }
    }
}

impl From<u8> for PolicyEngineVersion {
    fn from(version: u8) -> Self {
        match version {
            1 => PolicyEngineVersion::V1,
            _ => PolicyEngineVersion::V1,
        }
    }
}

impl From<Policy> for PolicyAccountType {
    fn from(ptype: Policy) -> Self {
        match ptype {
            Policy::IdentityApproval => PolicyAccountType::IdentityApproval,
            Policy::TransactionAmountLimit { .. } => PolicyAccountType::TransactionAmountLimit,
            Policy::TransactionAmountVelocity { .. } => {
                PolicyAccountType::TransactionAmountVelocity
            }
            Policy::TransactionCountVelocity { .. } => PolicyAccountType::TransactionCountVelocity,
        }
    }
}
