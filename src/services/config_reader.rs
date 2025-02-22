use std::error::Error;
use std::{env, fs};

use crate::models::config::Config;

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    let config_file_path = env::var("CONFIG_FILE_PATH").expect("$CONFIG_FILE_PATH is not set");
    let content_yaml = fs::read_to_string(config_file_path)?;
    let config = serde_yml::from_str::<Config>(&content_yaml)?;

    Ok(config)
}
