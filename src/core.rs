// Here the core stuff for tapet stuff that touches the filesystem
// and so on
use attohttpc;
use fs_extra;
use std::fs;
use std::io::Write;
use std::error::Error;
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

pub fn ensure_folders(config: &Config) -> Result<(), Box<dyn Error>> {
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

pub fn restore_background(config: &Config, state_path: &str) -> Result<(), Box<dyn Error>> {
    let state = config::retrieve_state(state_path)?;
    set_background(config, &state.current_wallpaper);
    Ok(())
}


pub fn download_image(config: &Config, url: &str) -> Result<(), Box<dyn Error>> {
    let response = attohttpc::get(url).send()?;
    let img_data = response.bytes().expect("couldn't get bytes from image");
    let image_filename = Path::new(url).file_name().expect("couldn't get filename from url").to_str().expect("Couldn't turn osString into string");
    let download_folder = String::from(shellexpand::tilde(&config.tapet.downloads_folder));
    let destination = format!("{}/{}", download_folder, image_filename);
    
    let mut file = fs::File::create(destination)?;
    file.write_all(&img_data)?;
    file.flush()?;

    Ok(())
} 

pub fn number_downloaded(config: &Config) -> Result<u32, Box<dyn Error>> {
    let folder_path = String::from(shellexpand::tilde(&config.tapet.downloads_folder));
    let mut paths: Vec<String> = Vec::new();
    for entry in fs::read_dir(&folder_path).expect("Couldn't find downloads_folder") {
        let entry = entry.expect("Couldn't find file");
        let path = entry.path();
        let strpath = path.clone().into_os_string().into_string().expect("couldn't get a string from the path").clone();
        paths.push(strpath);
    }
    Ok(paths.len() as u32)
}

fn cleanup_previous(config: &Config) -> Result<(), Box<dyn Error>> {
    let folder_path = String::from(shellexpand::tilde(&config.tapet.previous_folder));
    let keep = config.tapet.previous_keep as usize;

    let mut paths: Vec<(String, fs::Metadata)> = Vec::new();
    for entry in fs::read_dir(&folder_path).expect("Couldn't find favourite folder") {
        let entry = entry.expect("couldn't find file");
        let path = entry.path();
        let strpath = path.clone().into_os_string().into_string().expect("couldn't get a string from the path").clone();
        let metadata = path.metadata()?;
        paths.push((strpath, metadata));
    }

    if paths.len() > keep {
        let to_delete = paths.len() - keep;
        paths.sort_unstable_by(|(_, meta_a),(_, meta_b)|
                meta_a.created().unwrap().partial_cmp(&meta_b.created().unwrap()).unwrap_or(std::cmp::Ordering::Equal));
        let delete_me: Vec<String> = paths.iter()
                                        .take(to_delete)
                                        .map(|(filename, _)| filename.clone())
                                        .take(to_delete)
                                        .collect();
        for path in delete_me {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}

fn move_to_previous(file_path: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    let filename = Path::new(&file_path).file_name().expect("Could not get stored filename").to_str().expect("Couldn't turn osString to string");
    let folder_to = String::from(shellexpand::tilde(&config.tapet.previous_folder));
    let destination = format!("{}/{}", folder_to, filename);
    let copy_options = fs_extra::file::CopyOptions {overwrite: false, skip_exist: true, buffer_size: 64000};
    fs_extra::file::move_file(file_path, destination, &copy_options)?;

    cleanup_previous(config)?;

    Ok(())
}

pub fn copy_to_favorite(config: &Config, state_path: &str) -> Result<(), Box<dyn Error>> {
    let cur_state = config::retrieve_state(state_path)?;
    let cur_wp = cur_state.current_wallpaper;
    let filename = Path::new(&cur_wp).file_name().expect("could not get storted filename").to_str().expect("Couldn't turn osSrtring to string");
    let folder_to = String::from(shellexpand::tilde(&config.tapet.favorites_folder));
    let destination = format!("{}/{}", folder_to, filename);
    let copy_options = fs_extra::file::CopyOptions {overwrite: false, skip_exist: true, buffer_size: 64000};
    fs_extra::file::copy(cur_wp, destination, &copy_options)?;

    Ok(())
}

pub fn set_new_downloaded(config: &Config, state_path: &str) -> Result<(), Box<dyn Error>> {
    let cur_state = config::retrieve_state(state_path)?;
    let path_from = cur_state.current_wallpaper;
    let mut new_wp_path = path_from.clone();
    while new_wp_path == path_from {
        new_wp_path = random_downloaded(config);
    }

    if cur_state.is_downloaded {
        move_to_previous(&path_from, &config)?;
    }
    // Create a new state and set it
    let new_state = config::State {current_wallpaper: new_wp_path.clone(), is_downloaded: true, is_favorite: false};
    config::save_state(new_state, state_path)?;

    // And finally really set the wallpaper
    set_background(config, &new_wp_path);

    Ok(())
}

pub fn set_random_favorite(config: &Config, state_path: &str) -> Result<(), Box<dyn Error>> {
    let new_wp_path = random_favorite(config);
    let cur_state = config::retrieve_state(state_path)?;

    if cur_state.is_downloaded {
        let path_from = cur_state.current_wallpaper;
        move_to_previous(&path_from, &config)?;
    }

    let new_state = config::State {current_wallpaper: new_wp_path.clone(), is_downloaded: false, is_favorite: true};
    config::save_state(new_state, state_path)?;

    set_background(config, &new_wp_path);

    Ok(())
}
