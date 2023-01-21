use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseListDNS {
    pub result: Vec<ResultDNS>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultDNS {
    pub id: String,
    #[serde(rename = "zone_id")]
    pub zone_id: String,
    #[serde(rename = "zone_name")]
    pub zone_name: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: String,
    pub proxiable: bool,
    pub proxied: bool,
    pub ttl: i64,
    pub locked: bool,
}
