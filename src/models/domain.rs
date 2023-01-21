use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Domain {
    pub name: String,
    pub type_ip: String,
    pub proxied: bool,
    pub create: bool,
    pub zone_id: String,
}
