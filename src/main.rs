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
            Arg::with_name("update")
                .short("u")
                .long("update")
                .help("Updates new wallpapers")
                .takes_value(false),
        )
        .get_matches();

    // Get configuration file path
    let env_vars: HashMap<String, String> = env::vars().collect();
    let config_folder = env_vars.get("XDG_CONFIG_HOME").expect("Could not find $XDG_CONFIG_HOME");
    let relative_conf_path = "/tapet/tapet.toml";
    let config_string = format!("{}{}", config_folder, relative_conf_path);
    let configuration_file = Path::new(&config_string);
    
    // Parse configuration
    let configuration = config::parse_config(configuration_file)?;

    // Make sure folders are there
    core::ensure_folders(&configuration)?;

    // TEMP: 
    println!("Random favourite: {}", core::random_favorite(&configuration));

    Ok(())
}
