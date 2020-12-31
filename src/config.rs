use serde::Deserialize;
use std::path::Path;
use std::io::Error;

//main config struct
#[derive(Debug, Deserialize)]
pub struct Config {
    //tapet: Option<Tapet>,
    tapet: Tapet,
    wallhaven: Wallhaven,
}

#[derive(Debug, Deserialize)]
struct Tapet {
    favorites_folder: Option<String>,
    downloads_folder: Option<String>,
    previous_folder: Option<String>,
    previous_keep: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct Wallhaven {
    download_number: Option<u64>,
    tags: Option<String>,
}

pub fn parse_config(filepath: &Path) -> Result<Config, Error> {
    let config_file = std::fs::read_to_string(filepath)?;
    let config: Config = toml::from_str(&config_file)?;
    Ok(config)
}
