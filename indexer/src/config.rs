use std::fmt::{Display, Formatter};

use common::config::load_config_using_env_prefix;
use figment::value::Value;
use plerkle_messenger::MessengerConfig;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;
use std::env;
use tracing_subscriber::fmt;

use crate::error::IndexerError;

#[derive(Deserialize, PartialEq, Debug, Clone, Default)]
pub struct IndexerConfig {
    pub database_config: DatabaseConfig,
    pub messenger_config: MessengerConfig,
    pub env: Option<String>,
    pub rpc_config: RpcConfig,
    pub metrics_port: Option<u16>,
    pub metrics_host: Option<String>,
    pub backfiller: Option<bool>,
    pub max_postgres_connections: Option<u32>,
    pub account_stream_worker_count: Option<u32>,
    pub transaction_stream_worker_count: Option<u32>,
    pub code_version: Option<String>,
    pub pod_type: Option<PodType>,
}

impl IndexerConfig {
    pub fn get_database_url(&self) -> String {
        self.database_config
            .get(DATABASE_URL_KEY)
            .and_then(|u| u.clone().into_string())
            .ok_or(IndexerError::ConfigurationError {
                msg: format!("Database connection string missing: {}", DATABASE_URL_KEY),
            })
            .unwrap()
    }

    pub fn get_rpc_url(&self) -> String {
        self.rpc_config
            .get(RPC_URL_KEY)
            .and_then(|u| u.clone().into_string())
            .ok_or(IndexerError::ConfigurationError {
                msg: format!("RPC connection string missing: {}", RPC_URL_KEY),
            })
            .unwrap()
    }

    pub fn get_messenger_client_config(&self) -> MessengerConfig {
        let mut mc = self.messenger_config.clone();
        mc.connection_config
            .insert("consumer_id".to_string(), Value::from(rand_string()));
        mc
    }

    pub fn get_account_stream_worker_count(&self) -> u32 {
        self.account_stream_worker_count.unwrap_or(2)
    }
}

// Types and constants used for Figment configuration items.
pub type DatabaseConfig = figment::value::Dict;

pub const DATABASE_URL_KEY: &str = "url";
pub const DATABASE_LISTENER_CHANNEL_KEY: &str = "listener_channel";

pub type RpcConfig = figment::value::Dict;

pub const RPC_URL_KEY: &str = "url";
pub const RPC_COMMITMENT_KEY: &str = "commitment";
pub const CODE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum PodType {
    Regular,
    Backfiller,
}

impl Display for PodType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PodType::Regular => write!(f, "Regular"),
            PodType::Backfiller => write!(f, "Backfiller"),
        }
    }
}

pub fn rand_string() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

pub fn setup_config() -> IndexerConfig {
    let mut config: IndexerConfig = load_config_using_env_prefix("INGESTER_");
    config.code_version = Some(CODE_VERSION.to_string());
    config
}

pub fn init_logger() {
    let env_filter = env::var("RUST_LOG")
        .or::<Result<String, ()>>(Ok("info".to_string()))
        .unwrap();
    let t = tracing_subscriber::fmt().with_env_filter(env_filter);
    t.event_format(fmt::format::json()).init();
}
