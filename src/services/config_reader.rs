use std::error::Error;
use std::fs;

use crate::models::config::Config;

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    let content_yaml = fs::read_to_string("config/config.yaml")?;
    let config = serde_yaml::from_str::<Config>(&content_yaml)?;

    Ok(config)
}
