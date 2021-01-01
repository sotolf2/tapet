// Here the core stuff for tapet stuff that touches the filesystem
// and so on
use std::fs;
use std::io::Error;
use std::path::Path;
use std::process::Command;
use rand::seq::SliceRandom;
use crate::config::Config;
use crate::config;

fn random_from_folder(folder_str: &str) -> String{
    let folder_path = Path::new(&folder_str);
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

pub fn random_favorite(config: &Config) -> String {
    let favorite_string = String::from(shellexpand::tilde(&config.tapet.favorites_folder));
    random_from_folder(&favorite_string)
}

pub fn random_downloaded(config: &Config) -> String {
    let downloaded_string = String::from(shellexpand::tilde(&config.tapet.downloads_folder));
    random_from_folder(&downloaded_string)
}

fn set_with_feh(image_path: &str) {
    let status = Command::new("feh")
        .arg("--bg-scale")
        .arg(image_path)
        .status().expect("failed to execute feh are you sure it's installed and on the path?");
    assert!(status.success())
}
fn set_with_nitrogen(image_path: &str) {
    let status = Command::new("nitrogen")
        .arg("--set-scaled")
        .arg(image_path)
        .status().expect("failed to execute nitrogen are you sure it's installed and on the path?");
    assert!(status.success())
}

pub fn set_background(config: &Config, image_path: &str) {
    let setter = &config.tapet.wallpaper_setter;
    match setter.as_str() {
        "feh" => set_with_feh(image_path),
        "nitrogen" => set_with_nitrogen(image_path),
        _ => panic!("No known wallpaper setter set in config, (\"feh\", \"nitrogen\")"),
    }
} 

pub fn ensure_folders(config: &Config) -> Result<(), Error> {
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

pub fn restore_background(config: &Config, state_path: &str) -> Result<(),Error> {
    let state = config::retrieve_state(state_path)?;
    set_background(config, &state.current_wallpaper);
    Ok(())
}
