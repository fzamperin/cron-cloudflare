mod models;
mod services;

use std::sync::Mutex;

use models::config::Config;
use once_cell::sync::Lazy;
use services::{config_reader::get_config, cron};

static FILE_CONFIG: Lazy<Mutex<Vec<Config>>> = Lazy::new(|| Mutex::new(vec![]));

fn main() {
    read_config();
    cron::start_cron();
}

fn read_config() {
    let config = get_config().unwrap();
    FILE_CONFIG.lock().unwrap().push(config);
}
