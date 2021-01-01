use serde::Deserialize;
use std::path::Path;
use std::io::Error;

//main config struct
#[derive(Debug, Deserialize)]
pub struct Config {
    //tapet: Option<Tapet>,
    pub tapet: Tapet,
    pub wallhaven: Wallhaven,
}

#[derive(Debug, Deserialize)]
pub struct Tapet {
    pub favorites_folder: String,
    pub downloads_folder: String,
    pub previous_folder: String,
    pub previous_keep: u64,
    pub wallpaper_setter: String,
}

#[derive(Debug, Deserialize)]
pub struct Wallhaven {
    pub download_number: u64,
    pub tags: String,
}

pub fn parse_config(filepath: &Path) -> Result<Config, Error> {
    let config_file = std::fs::read_to_string(filepath)?;
    let config: Config = toml::from_str(&config_file)?;
    Ok(config)
}
