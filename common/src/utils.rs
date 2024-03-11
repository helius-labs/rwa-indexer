use std::path::PathBuf;

use git2::Repository;
use solana_program::{pubkey, pubkey::Pubkey};

pub const APPROVAL_ACCOUNT_LEN: u64 = 136;
pub const TRACKER_ACCOUNT_LEN: u64 = 240;
pub const IDENTITY_ACCOUNT_LEN: u64 = 83;
pub const DATA_ACCOUNT_LEN: u64 = 105;
pub const IDENTITY_APPROVAL_LEN: u64 = 20;
pub const TRANSACATION_AMOUNT_LIMIT_LEN: u64 = 28;
pub const TRANSACATION_AMOUNT_VELOCITY_LEN: u64 = 36;
pub const TRANSACATION_COUNT_VELOCITY_LEN: u64 = 36;

pub fn get_relative_git_path(file_path: &str) -> PathBuf {
    let repo = Repository::discover(".").expect("Failed to discover Git repository");
    let git_root = repo.workdir().expect("Failed to get Git repository root");

    git_root.join(file_path)
}

pub const ASSET_CONTROLLER_PROGRAM_ID: Pubkey =
    pubkey!("acpcFrzEYKjVLvZGWueTV8vyDjhu3oKC7sN38QELLan");

pub const DATA_REGISTRY_PROGRAM_ID: Pubkey = pubkey!("dataeP5X1e7XsWN1ovDSEDP5cqaEUnKBmHE5iZhXPVw");

pub const IDENTIFIER_REGISTRY_PROGRAM_ID: Pubkey =
    pubkey!("idtynCMYbdisCTv4FrCWPSQboZb1uM4TV2cPi79yxQf");

pub const POLICY_ENGINE_PROGRAM_ID: Pubkey = pubkey!("po1cPf1eyUJJPqULw4so3T4JU9pdFn83CDyuLEKFAau");

pub fn find_asset_controller_pda(mint: &Pubkey) -> (solana_program::pubkey::Pubkey, u8) {
    solana_program::pubkey::Pubkey::find_program_address(
        &[mint.as_ref()],
        &ASSET_CONTROLLER_PROGRAM_ID,
    )
}

pub fn find_data_registry_pda(data_type: &Pubkey) -> (solana_program::pubkey::Pubkey, u8) {
    solana_program::pubkey::Pubkey::find_program_address(
        &[data_type.as_ref()],
        &DATA_REGISTRY_PROGRAM_ID,
    )
}

pub fn find_identifier_registry_pda(identifier: &Pubkey) -> (solana_program::pubkey::Pubkey, u8) {
    solana_program::pubkey::Pubkey::find_program_address(
        &[identifier.as_ref()],
        &IDENTIFIER_REGISTRY_PROGRAM_ID,
    )
}

pub fn find_policy_engine_pda(policy: &Pubkey) -> (solana_program::pubkey::Pubkey, u8) {
    solana_program::pubkey::Pubkey::find_program_address(
        &[policy.as_ref()],
        &POLICY_ENGINE_PROGRAM_ID,
    )
}
