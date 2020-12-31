use toml::de;
use serde::Deserialize;
use std::path::Path;
use std::fs;
use std::io;
use std::io::Error;

//main config struct
#[derive(Debug, Deserialize)]
pub struct Config {
    tapet: Option<TapetConfig>,
    wallhaven: Option<WallhavenConfig>,
}

#[derive(Debug, Deserialize)]
struct TapetConfig {
    favorites_folder: Option<String>,
    downloads_folder: Option<String>,
    previous_folder: Option<String>,
    previous_keep: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct WallhavenConfig {
    download_number: Option<u64>,
    tags: Option<String>,
}

pub fn parse_config(filepath: &Path) -> Result<Config, Error> {
    let config_file = std::fs::read_to_string(filepath)?;
    let config: Config = toml::de::from_str(&config_file)?;
    Ok(config)
}
