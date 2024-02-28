use crate::utils::get_relative_git_path;
use figment::providers::Format;
use figment::{
    providers::{Env, Json},
    Figment,
};
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};

fn get_local_config_file_path() -> PathBuf {
    let file_path = get_relative_git_path("common/src/local_config.json");
    if !Path::new(&file_path).exists() {
        panic!(
            "Configuration file does not exist: {}",
            file_path.to_string_lossy()
        );
    }
    file_path
}

pub fn load_config_using_env_prefix<T: DeserializeOwned>(env_prefix: &str) -> T {
    let mut config = Figment::new().join(Env::prefixed(env_prefix));
    if let Ok("local") = std::env::var("ENV").as_deref() {
        config = config.join(Json::file(get_local_config_file_path()));
    }
    config.extract::<T>().unwrap()
}

pub fn load_local_config<T: DeserializeOwned>() -> T {
    let config = Figment::new().join(Json::file(get_local_config_file_path()));
    config.extract::<T>().unwrap()
}
