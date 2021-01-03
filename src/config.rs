use serde::{Deserialize, Serialize};
use std::path::Path;
use std::io::Write;
use std::fs;
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
    pub history: u32,
    pub interval: u64,
    pub download_every: u64,
}

#[derive(Debug, Deserialize)]
pub struct Wallhaven {
    pub download_number: u32,
    pub tags: String,
}

pub fn parse_config(filepath: &Path) -> Result<Config, Error> {
    let config_file = std::fs::read_to_string(filepath)?;
    let config: Config = toml::from_str(&config_file)?;
    Ok(config)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct State {
    pub current_wallpaper: String,
    pub is_favorite: bool,
    pub is_downloaded: bool,
}

pub fn save_state(state: State, filepath: &str) -> Result<(), Error> {
    let mut file = fs::File::create(filepath)?;
    let state_json = serde_json::to_string(&state)?; 
    file.write_all(state_json.as_bytes())?;
    file.flush()?;
    
    Ok(())
}

pub fn retrieve_state(filepath: &str) -> Result<State, Error> {
    let state_json = fs::read_to_string(filepath)?;
    let state: State = serde_json::from_str(&state_json)?;
    Ok(state)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    pub urls: Vec<String>,
}

pub fn retrive_history(filepath: &str) -> Result<History, Error> {
    // create an empty one if it doesn't exist
    let history_path = Path::new(filepath);
    if !history_path.exists() {
        let empty = History {urls: Vec::new()};
        save_history(empty, filepath)?;
    }

    let history_json = fs::read_to_string(filepath)?;
    let history: History = serde_json::from_str(&history_json)?;
    Ok(history)
}

pub fn save_history(history: History, filepath: &str) -> Result<(), Error> {
    let mut file = fs::File::create(filepath)?;
    let history_json = serde_json::to_string(&history)?; 
    file.write_all(history_json.as_bytes())?;
    file.flush()?;
    
    Ok(())
}

pub fn append_history(config: &Config, filepath: &str, urls: Vec<&String>) -> Result<(), Error> {
    let limit = config.tapet.history as usize;
    let mut history = retrive_history(filepath)?.urls;
    
    for url in urls {
        history.push(String::from(url));
    }
    
    let length_history = history.len();
    let new_urls = 
        if length_history > limit {
            history.get(length_history-limit..length_history).unwrap()
        } else {
            &history
        };

    let new = History {urls: (*new_urls).to_vec()};
    save_history(new, filepath)?;

    Ok(())
}
