use super::domain::Domain;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub auth: String,
    pub domains: Vec<Domain>,
}
