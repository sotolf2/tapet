// Here the core stuff for tapet stuff that touches the filesystem
// and so on
use std::fs;
use std::io::Error;
use std::path::Path;
use crate::config::Config;

pub fn ensure_folders(config: Config) -> Result<(), Error>{
    let tapet = config.tapet;
    let folders = vec![
        tapet.favorites_folder,
        tapet.downloads_folder,
        tapet.previous_folder,
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
