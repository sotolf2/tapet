// Here the core stuff for tapet stuff that touches the filesystem
// and so on
use std::fs;
use std::io::Error;
use std::path::Path;
use rand::seq::SliceRandom;
use crate::config::Config;


pub fn random_favorite(config: &Config) -> String {
    let favorite_string = String::from(shellexpand::tilde(&config.tapet.favorites_folder));
    let folder_path = Path::new(&favorite_string);
    let mut paths: Vec<String> = Vec::new();
    for entry in fs::read_dir(&folder_path).expect("Couldn't find favourite folder") {
        let entry = entry.expect("couldn't find file");
        let path = entry.path();
        paths.push(path.into_os_string().into_string().expect("couldn't get a string from the path"));
    }
    let path = paths.choose(&mut rand::thread_rng());
    match path {
        None => String::from(""),
        Some(str) => String::from(str)
    }
}

pub fn ensure_folders(config: &Config) -> Result<(), Error>{
    let tapet = &config.tapet;
    let folders = vec![
        &tapet.favorites_folder,
        &tapet.downloads_folder,
        &tapet.previous_folder,
    ];
    
    for folder in folders {
        let folder_string = String::from(shellexpand::tilde(&folder));
        let folder_path = Path::new(&folder_string);
        if !folder_path.exists() {
            fs::create_dir_all(folder_path)?;
        }
    }
    Ok(())
}
