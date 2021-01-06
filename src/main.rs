use clap::{App, Arg};
use std::env;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::time::Duration;
use std::thread;
mod config;
mod core;
mod wallhaven;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse arguements
    let matches = App::new("Tapet")
        .version("0.1")
        .author("Sotolf")
        .about("A wallpaper switcher helper")
        .arg(
            Arg::with_name("next")
                .short("n")
                .long("next")
                .help("sets the next wallpaper")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("favorite")
                .short("f")
                .long("favorite")
                .help("Saves the current wallpaper in the favorites")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("random")
            .short("r")
            .long("random-favourite")
            .help("Set a random wallpaper from the favourites folder")
            .takes_value(false),
        )
        .arg(
            Arg::with_name("restore")
            .short("R")
            .long("restore")
            .help("Restores current wallpaper")
            .takes_value(false),
        )
        .arg(
            Arg::with_name("update")
                .short("u")
                .long("update")
                .help("Updates new wallpapers")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("daemon")
            .short("d")
            .long("daemon")
            .help("runs in the background and updates wallpaper automatically")
            .takes_value(false),
        )
        .get_matches();

    // Get configuration file path
    let env_vars: HashMap<String, String> = env::vars().collect();
    let config_home = env_vars.get("XDG_CONFIG_HOME").expect("Could not find $XDG_CONFIG_HOME");
    let conf_path = format!("{}{}", config_home, "/tapet/");
    let configuration_file = "tapet.toml";
    let state_file = "state.json";
    let history_file = "history.json";
    let config_string = format!("{}{}", conf_path, configuration_file);
    let configuration_file = Path::new(&config_string);
    let state_path = format!("{}{}", conf_path, state_file);
    let history_path = format!("{}{}", conf_path, history_file);

    
    // Parse configuration
    let configuration = config::parse_config(configuration_file)?;

    // Make sure folders are there
    core::ensure_folders(&configuration)?;

    // Go through and do the desired things
    if matches.is_present("next") {
        core::set_new_downloaded(&configuration, &state_path)?;
    }
    if matches.is_present("favorite") {
        core::copy_to_favorite(&configuration, &state_path)?;
    }
    if matches.is_present("random") {
        core::set_random_favorite(&configuration, &state_path)?;
    }
    if matches.is_present("restore") {
        core::restore_background(&configuration, &state_path)?;
    }
    if matches.is_present("update") {
        wallhaven::download_images(&configuration, &history_path)?;
    }
    if matches.is_present("daemon") {
        let sleep_min = configuration.tapet.interval;
        let sleep_duration = Duration::from_secs(sleep_min * 60);
        let counter_lim = configuration.tapet.download_every;
        let mut counter = 0;
        loop {
            core::set_new_downloaded(&configuration, &state_path)?;
            if counter == counter_lim {
                wallhaven::download_images(&configuration, &history_path)?;
                counter = 0;
            }
            counter += 1;
            thread::sleep(sleep_duration);
        }
    }
    Ok(())
}
