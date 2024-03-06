use super::sea_orm_active_enums::{
    AssetControllerVersion, DataAccountType, DataRegistryVersion, IdentityAccountVersion,
    IdentityApprovalVersion, IdentityRegistryVersion, PolicyAccountVersion,
    TransactionAmountLimitVersion, TransactionAmountVelocityVersion,
    TransactionCountVelocityVersion,
};
use data_registry::state::DataAccountType as ProgramDataAccountType;

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

impl From<u8> for PolicyAccountVersion {
    fn from(version: u8) -> Self {
        match version {
            1 => PolicyAccountVersion::V1,
            _ => PolicyAccountVersion::V1,
        }
    }
}

impl From<u8> for IdentityApprovalVersion {
    fn from(version: u8) -> Self {
        match version {
            1 => IdentityApprovalVersion::V1,
            _ => IdentityApprovalVersion::V1,
        }
    }
}

impl From<u8> for TransactionAmountLimitVersion {
    fn from(version: u8) -> Self {
        match version {
            1 => TransactionAmountLimitVersion::V1,
            _ => TransactionAmountLimitVersion::V1,
        }
    }
}

impl From<u8> for TransactionAmountVelocityVersion {
    fn from(version: u8) -> Self {
        match version {
            1 => TransactionAmountVelocityVersion::V1,
            _ => TransactionAmountVelocityVersion::V1,
        }
    }
}

impl From<u8> for TransactionCountVelocityVersion {
    fn from(version: u8) -> Self {
        match version {
            1 => TransactionCountVelocityVersion::V1,
            _ => TransactionCountVelocityVersion::V1,
        }
    }
}
