use clap::{App, Arg};
use std::env;
use std::collections::HashMap;
use std::path::Path;
use std::io::Error;
mod config;
mod core;

fn main() -> Result<(), Error> {
    // Parse arguements
    let matches = App::new("tapet")
        .version("0.1")
        .author("Sotolf")
        .about("A wallpaper helper")
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
        .get_matches();

    // Get configuration file path
    let env_vars: HashMap<String, String> = env::vars().collect();
    let config_home = env_vars.get("XDG_CONFIG_HOME").expect("Could not find $XDG_CONFIG_HOME");
    let conf_path = format!("{}{}", config_home, "/tapet/");
    let configuration_file = "tapet.toml";
    let state_file = "state.json";
    let config_string = format!("{}{}", conf_path, configuration_file);
    let configuration_file = Path::new(&config_string);
    let state_path = format!("{}{}", conf_path, state_file);
    
    // Parse configuration
    let configuration = config::parse_config(configuration_file)?;

    // Make sure folders are there
    core::ensure_folders(&configuration)?;

    // TEMP: 
    let rand_fav = core::random_favorite(&configuration);
    println!("Random favourite: {}", rand_fav);
    let state = config::State {current_wallpaper: rand_fav.clone(), is_favorite: true};
    core::set_background(&configuration, &rand_fav);
    println!("Saving state: {:?} in: {}", state, &state_path);
    config::save_state(state, &state_path)?;
    let state2 = config::retrieve_state(&state_path)?;
    println!("Retrieved state: {:?}", state2);


    Ok(())
}
