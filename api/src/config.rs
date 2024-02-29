use common::config::load_config_using_env_prefix;

use serde::Deserialize;

#[derive(Deserialize, Clone, Default)]
pub struct Config {
    pub database_url: String,
    pub metrics_port: Option<u16>,
    pub metrics_host: Option<String>,
    pub server_port: u16,
    pub env: Option<String>,
    pub db_max_conn: Option<u32>,
}

pub fn load_config() -> Config {
    load_config_using_env_prefix("APP_")
}
