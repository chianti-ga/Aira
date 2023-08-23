use std::error::Error;
use std::fs;
use std::path::PathBuf;

use rfd::{MessageDialog, MessageLevel};

use crate::tools_utils::ToolsPaths;

pub fn load_from_config() -> ToolsPaths {
    let config_file = fs::read_to_string("config.toml");
    let tools_paths: ToolsPaths = match config_file {
        Ok(file) => toml::from_str(&file).unwrap(),
        Err(_) => ToolsPaths {
            vtex2: PathBuf::new(),
            studio_mdl: PathBuf::new(),
            gmad: PathBuf::new(),
        }
    };
    tools_paths
}

pub fn save_to_config(tools_paths: &ToolsPaths) {
    match toml::to_string_pretty(tools_paths) {
        Ok(string) => {
            match fs::write("config.toml", string) {
                Ok(_) => {}
                Err(err) => {
                    MessageDialog::new()
                        .set_title("Error")
                        .set_level(MessageLevel::Error)
                        .set_description(&*format!("Cannot save configuration to file :\n {}", err.description()))
                        .show();
                }
            }
        }
        Err(err) => {
            MessageDialog::new()
                .set_title("Error")
                .set_level(MessageLevel::Error)
                .set_description(&*format!("Cannot serialize :\n {}", err.description()))
                .show();
        }
    }
}