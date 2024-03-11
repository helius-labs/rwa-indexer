use std::env;
use std::path::Path;

use std::str::FromStr;

use common::utils;
use rwa_api::api::RwaApi;

use rwa_api::config::Config;

use migration::sea_orm::{
    ConnectionTrait, DatabaseConnection, ExecResult, SqlxPostgresConnector, Statement,
};
use migration::{Migrator, MigratorTrait};

use indexer::{config::DatabaseConfig, config::IndexerConfig, program_handlers::ProgramHandler};
use once_cell::sync::Lazy;
use plerkle_serialization::root_as_account_info;
use plerkle_serialization::serializer::serialize_account;
use plerkle_serialization::solana_geyser_plugin_interface_shims::ReplicaAccountInfoV2;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;

use futures_util::StreamExt as FuturesStreamExt;
use futures_util::TryStreamExt;
use tokio_stream::{self as stream};

use log::error;
// use rand::seq::SliceRandom;
use serde::de::DeserializeOwned;
use solana_account_decoder::{UiAccount, UiAccountEncoding};
use solana_client::{
    client_error::Result as RpcClientResult, rpc_config::RpcAccountInfoConfig,
    rpc_request::RpcRequest, rpc_response::Response as RpcResponse,
};
use solana_sdk::{
    account::Account,
    commitment_config::{CommitmentConfig, CommitmentLevel},
};
use std::time::Duration;

use std::path::PathBuf;

use tokio::time::sleep;

pub const DEFAULT_SLOT: u64 = 1;

pub struct TestSetup {
    pub name: String,
    pub client: RpcClient,
    pub db: Arc<DatabaseConnection>,
    pub transformer: ProgramHandler,
    pub rwa_api: RwaApi,
    pub config: IndexerConfig,
}

impl TestSetup {
    pub async fn new(name: String) -> Self {
        Self::new_with_options(name, TestSetupOptions::default()).await
    }

    pub async fn new_with_options(name: String, opts: TestSetupOptions) -> Self {
        let database_test_url = std::env::var("DATABASE_TEST_URL").unwrap();
        let mut database_config = DatabaseConfig::new();
        database_config.insert("database_url".to_string(), database_test_url.clone().into());

        let indexer_config: IndexerConfig = IndexerConfig {
            database_config,
            ..IndexerConfig::default()
        };

        if !(database_test_url.contains("localhost") || database_test_url.contains("127.0.0.1")) {
            panic!("Tests can only be run on a local database");
        }

        let pool = setup_pg_pool(database_test_url.clone()).await;
        let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone());

        let transformer = ProgramHandler::new(pool.clone(), indexer_config.clone());

        let rpc_url = match opts.network.unwrap_or_default() {
            Network::Mainnet => std::env::var("MAINNET_RPC_URL").unwrap(),
            Network::Devnet => std::env::var("DEVNET_RPC_URL").unwrap(),
        };
        let client = RpcClient::new(rpc_url.to_string());

        let rwa_api_config: Config = rwa_api::config::Config {
            database_url: database_test_url.to_string(),
            ..Default::default()
        };
        let rwa_api = rwa_api::api::RwaApi::from_config(rwa_api_config)
            .await
            .unwrap();

        TestSetup {
            name,
            client,
            db: Arc::new(db),
            transformer,
            rwa_api,
            config: indexer_config,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct TestSetupOptions {
    pub network: Option<Network>,
}

pub async fn setup_pg_pool(database_url: String) -> PgPool {
    let options: PgConnectOptions = database_url.parse().unwrap();
    PgPoolOptions::new()
        .min_connections(1)
        .connect_with(options)
        .await
        .unwrap()
}

pub async fn truncate_table(
    db: Arc<DatabaseConnection>,
    table: String,
) -> Result<migration::sea_orm::ExecResult, migration::DbErr> {
    let raw_sql = format!("TRUNCATE TABLE {} CASCADE", table);
    db.execute(Statement::from_string(db.get_database_backend(), raw_sql))
        .await
}

static INIT: Lazy<Mutex<Option<()>>> = Lazy::new(|| Mutex::new(None));

fn setup_logging() {
    let env_filter = env::var("RUST_LOG").unwrap_or("debug,sqlx::off".to_string());
    tracing_subscriber::fmt()
        .with_test_writer()
        .with_env_filter(env_filter)
        .init();
}

pub async fn apply_migrations_and_delete_data(db: Arc<DatabaseConnection>) {
    let mut init = INIT.lock().await;
    if init.is_none() {
        Migrator::fresh(&db).await.unwrap();
        *init = Some(());
        setup_logging();
        // Mutex will dropped once it goes out of scope.
        return;
    }

    let tables: Vec<String> = db
        .query_all(Statement::from_string(db.get_database_backend(), "SELECT tablename FROM pg_catalog.pg_tables WHERE schemaname != 'pg_catalog' AND schemaname != 'information_schema' AND tablename != 'seaql_migrations'".to_string()))
        .await
        .unwrap().into_iter()
        .map(|row| row.try_get("", "tablename").unwrap()).collect::<Vec<String>>();

    let max_concurrency = 10;

    stream::iter(tables.into_iter())
        .map(|table| truncate_table(db.clone(), table.clone()))
        .buffer_unordered(max_concurrency)
        .try_collect::<Vec<ExecResult>>()
        .await
        .unwrap();
}

// Util functions for accounts
pub async fn rpc_tx_with_retries<T, E>(
    client: &RpcClient,
    request: RpcRequest,
    value: serde_json::Value,
    max_retries: u8,
    error_key: E,
) -> RpcClientResult<T>
where
    T: DeserializeOwned,
    E: std::fmt::Debug,
{
    let mut retries = 0;
    let mut delay = Duration::from_millis(500);
    loop {
        match client.send(request, value.clone()).await {
            Ok(value) => return Ok(value),
            Err(error) => {
                if retries < max_retries {
                    error!("retrying {request} {error_key:?}: {error}");
                    sleep(delay).await;
                    delay *= 2;
                    retries += 1;
                } else {
                    return Err(error);
                }
            }
        }
    }
}

pub async fn fetch_account(
    pubkey: Pubkey,
    client: &RpcClient,
    max_retries: u8,
) -> anyhow::Result<(Account, u64)> {
    const CONFIG: RpcAccountInfoConfig = RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::Base64Zstd),
        commitment: Some(CommitmentConfig {
            commitment: CommitmentLevel::Finalized,
        }),
        data_slice: None,
        min_context_slot: None,
    };

    let response: RpcResponse<Option<UiAccount>> = rpc_tx_with_retries(
        client,
        RpcRequest::GetAccountInfo,
        serde_json::json!([pubkey.to_string(), CONFIG]),
        max_retries,
        pubkey,
    )
    .await?;

    let account: Account = response
        .value
        .ok_or_else(|| anyhow::anyhow!("failed to get account {pubkey}"))?
        .decode()
        .ok_or_else(|| anyhow::anyhow!("failed to parse account {pubkey}"))?;

    Ok((account, response.context.slot))
}

pub async fn fetch_and_serialize_account(
    client: &RpcClient,
    pubkey: Pubkey,
    slot: Option<u64>,
) -> anyhow::Result<Vec<u8>> {
    let max_retries = 5;

    let fetch_result = fetch_account(pubkey, &client, max_retries).await;

    let (account, actual_slot) = match fetch_result {
        Ok((account, actual_slot)) => (account, actual_slot),
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to fetch account: {:?}", e));
        }
    };

    let fbb = flatbuffers::FlatBufferBuilder::new();
    let account_info = ReplicaAccountInfoV2 {
        pubkey: &pubkey.to_bytes(),
        lamports: account.lamports,
        owner: &account.owner.to_bytes(),
        executable: account.executable,
        rent_epoch: account.rent_epoch,
        data: &account.data,
        write_version: 0,
        txn_signature: None,
    };
    let is_startup = false;

    let fbb = serialize_account(
        fbb,
        &account_info,
        match slot {
            Some(slot) => slot,
            None => actual_slot,
        },
        is_startup,
    );
    Ok(fbb.finished_data().to_vec())
}

pub async fn index_account_bytes(setup: &TestSetup, account_bytes: Vec<u8>) {
    let account = root_as_account_info(&account_bytes).unwrap();

    setup
        .transformer
        .handle_account_update(account, &setup.config)
        .await
        .unwrap();
}

pub async fn cached_fetch_account(
    setup: &TestSetup,
    account: Pubkey,
    slot: Option<u64>,
) -> Vec<u8> {
    cached_fetch_account_with_error_handling(setup, account, slot)
        .await
        .unwrap()
}

fn get_relative_project_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path)
}

async fn cached_fetch_account_with_error_handling(
    setup: &TestSetup,
    account: Pubkey,
    slot: Option<u64>,
) -> anyhow::Result<Vec<u8>> {
    let dir = get_relative_project_path(&format!("tests/data/accounts/{}", setup.name));

    if !Path::new(&dir).exists() {
        std::fs::create_dir(&dir).unwrap();
    }
    let file_path = dir.join(account.to_string());

    if file_path.exists() {
        Ok(std::fs::read(file_path).unwrap())
    } else {
        let account_bytes = fetch_and_serialize_account(&setup.client, account, slot).await?;
        std::fs::write(file_path, &account_bytes).unwrap();
        Ok(account_bytes)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SeedEvent {
    Account(Pubkey),
    TokenMint(Pubkey),
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Network {
    #[default]
    Mainnet,
    Devnet,
}

#[derive(Clone, Copy, Debug)]
pub enum Order {
    Forward,
    AllPermutations,
}

pub async fn index_seed_events(setup: &TestSetup, events: Vec<&SeedEvent>) {
    for event in events {
        match event {
            SeedEvent::Account(account) => {
                index_account_with_ordered_slot(setup, *account).await;
            }
            SeedEvent::TokenMint(mint) => {
                index_token_mint(setup, *mint).await;
            }
        }
    }
}

pub fn seed_account(str: &str) -> SeedEvent {
    SeedEvent::Account(Pubkey::from_str(str).unwrap())
}

pub fn seed_token_mint(str: &str) -> SeedEvent {
    SeedEvent::TokenMint(Pubkey::from_str(str).unwrap())
}

pub fn seed_accounts<I>(strs: I) -> Vec<SeedEvent>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    strs.into_iter().map(|s| seed_account(s.as_ref())).collect()
}

pub fn seed_token_mints<I>(strs: I) -> Vec<SeedEvent>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    strs.into_iter()
        .map(|s| seed_token_mint(s.as_ref()))
        .collect()
}

pub async fn index_account(setup: &TestSetup, account: Pubkey) {
    // If we used different slots for accounts, then it becomes harder to test updates of related
    // accounts because we need to factor the fact that some updates can be disregarded because
    // they are "stale".
    let slot = Some(DEFAULT_SLOT);
    let account_bytes = cached_fetch_account(setup, account, slot).await;
    index_account_bytes(setup, account_bytes).await;
}

async fn index_account_with_ordered_slot(setup: &TestSetup, account: Pubkey) {
    let slot = None;
    let account_bytes = cached_fetch_account(setup, account, slot).await;
    index_account_bytes(setup, account_bytes).await;
}

async fn index_token_mint(setup: &TestSetup, mint: Pubkey) {
    index_account(setup, mint).await;

    // If we used different slots for accounts, then it becomes harder to test updates of related
    // accounts because we need to factor the fact that some updates can be disregarded because
    // they are "stale".
    let slot = Some(1);

    let asset_controller_pda = utils::find_asset_controller_pda(&mint).0;
    let data_pda = utils::find_data_registry_pda(&mint).0;
    let identifier_pda = utils::find_identifier_registry_pda(&mint).0;
    let policy_pda = utils::find_policy_engine_pda(&mint).0;

    if let Ok(account_bytes) =
        cached_fetch_account_with_error_handling(setup, asset_controller_pda, slot).await
    {
        index_account_bytes(setup, account_bytes).await;
    }

    if let Ok(account_bytes) = cached_fetch_account_with_error_handling(setup, data_pda, slot).await
    {
        index_account_bytes(setup, account_bytes).await;
    }

    if let Ok(account_bytes) =
        cached_fetch_account_with_error_handling(setup, identifier_pda, slot).await
    {
        index_account_bytes(setup, account_bytes).await;
    }

    if let Ok(account_bytes) =
        cached_fetch_account_with_error_handling(setup, policy_pda, slot).await
    {
        index_account_bytes(setup, account_bytes).await;
    }
}

pub fn trim_test_name(name: &str) -> String {
    name.replace("test_", "")
}

pub fn get_max_slot() -> u64 {
    // If you use any larger slot, you'll encounter overflow behavior.
    u64::MAX / 2
}
