use crate::models::{
    cloudflare_response::{ResponseListDNS, ResultDNS},
    config::Config,
    domain::Domain,
};
use chrono::Local;
use reqwest::blocking::Client;
use std::{collections::HashMap, error::Error};

pub fn update_domains(config: &Config) {
    let client = reqwest::blocking::Client::new();
    for domain in &config.domains {
        update_domain(&config.auth, &domain, &client).unwrap();
        println!(
            "[{}], Finished processing domain entry: {}, type: {}",
            Local::now().format("%d/%m/%Y %H:%M:%S"),
            domain.name,
            domain.type_ip
        );
    }
}

fn update_domain(auth: &str, domain: &Domain, client: &Client) -> Result<(), Box<dyn Error>> {
    let ip_type = if domain.type_ip == "A" { "v4" } else { "v6" };

    let current_ip: String = client
        .get(format!("https://{}.ident.me", ip_type))
        .send()?
        .text()?;

    let dns_response = list_dns_request(auth, domain, &client)?;
    let dns_response = &dns_response.result[0];

    if current_ip == dns_response.content {
        println!(
            "[{}], Domain entry: {}, type: {}, does not need to be updated",
            Local::now().format("%d/%m/%Y %H:%M:%S"),
            domain.name,
            domain.type_ip
        );
    } else {
        println!(
            "[{}], Updating domain entry: {}, type: {}, to: {}",
            Local::now().format("%d/%m/%Y %H:%M:%S"),
            domain.name,
            domain.type_ip,
            current_ip
        );
        update_ip(client, auth, &dns_response, &current_ip)?;
    }

    Ok(())
}

fn update_ip(
    client: &Client,
    auth: &str,
    result_dns: &ResultDNS,
    new_ip: &str,
) -> Result<(), Box<dyn Error>> {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("content", new_ip);

    client
        .patch(format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
            result_dns.zone_id, result_dns.id
        ))
        .bearer_auth(auth)
        .json(&map)
        .send()?;

    Ok(())
}

fn list_dns_request(
    auth: &str,
    domain: &Domain,
    client: &Client,
) -> Result<ResponseListDNS, Box<dyn Error>> {
    let cloudflare_list_response = client
        .get(format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
            domain.zone_id
        ))
        .bearer_auth(auth)
        .query(&[("name", &domain.name), ("type", &domain.type_ip)])
        .send()?
        .json::<ResponseListDNS>()?;

    Ok(cloudflare_list_response)
}
