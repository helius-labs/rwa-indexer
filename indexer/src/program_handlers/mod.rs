use crate::{config::IndexerConfig, error::IndexerError};
use log::{debug, error, info};
use plerkle_serialization::{AccountInfo, Pubkey as FBPubkey};
use sea_orm::{DatabaseConnection, SqlxPostgresConnector};
use solana_sdk::pubkey::Pubkey;
use sqlx::PgPool;
use std::collections::{HashMap, HashSet, VecDeque};
use tokio::sync::mpsc::UnboundedSender;
use transformer::{
    program_handler::ProgramParser,
    programs::{asset_controller::AssetControllerParser, ProgramParseResult},
};

use crate::program_handlers::asset_controller::handle_asset_controller_program_account;

mod asset_controller;
mod data_registry;
mod identity_registry;
mod policy_engine;
mod utils;

pub struct ProgramHandler {
    storage: DatabaseConnection,
    matchers: HashMap<Pubkey, Box<dyn ProgramParser>>,
    key_set: HashSet<Pubkey>,
    config: IndexerConfig,
}

impl ProgramHandler {
    pub fn new(pool: PgPool, config: IndexerConfig) -> Self {
        let mut matchers: HashMap<Pubkey, Box<dyn ProgramParser>> = HashMap::with_capacity(1);
        let asset_controller = AssetControllerParser {};
        matchers.insert(asset_controller.key(), Box::new(asset_controller));
        let hs = matchers.iter().fold(HashSet::new(), |mut acc, (k, _)| {
            acc.insert(*k);
            acc
        });
        let pool: PgPool = pool;
        ProgramHandler {
            storage: SqlxPostgresConnector::from_sqlx_postgres_pool(pool),
            matchers,
            key_set: hs,
            config,
        }
    }

    pub fn match_program(&self, key: &FBPubkey) -> Option<&Box<dyn ProgramParser>> {
        self.matchers
            .get(&Pubkey::try_from(key.0.as_slice()).unwrap())
    }

    pub async fn handle_account_update<'b>(
        &self,
        acct: AccountInfo<'b>,
        config: &IndexerConfig,
    ) -> Result<(), IndexerError> {
        let owner = acct.owner().unwrap();
        if let Some(program) = self.match_program(owner) {
            let result = program.handle_account(&acct)?;
            let concrete = result.result_type();
            match concrete {
                ProgramParseResult::AssetControllerProgram(parsing_result) => {
                    handle_asset_controller_program_account(
                        &acct,
                        parsing_result,
                        &self.storage,
                        config,
                    )
                    .await
                    .map_err(|err| {
                        error!(
                            "Failed to handle token account {:?}: {:?}",
                            bs58::encode(acct.pubkey().unwrap().0.as_slice()).into_string(),
                            err
                        );
                        err
                    })
                }
                _ => Err(IndexerError::NotImplemented),
            }?;
        }
        Ok(())
    }
}
