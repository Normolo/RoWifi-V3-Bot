use reqwest::{Client, ClientBuilder, header};
use serde_json::Value;
use super::error::RoError;

#[derive(Clone)]
pub struct Patreon {
    client: Client
}

impl Patreon {
    pub fn new(patreon_key: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(patreon_key).unwrap());
        let client = ClientBuilder::new().default_headers(headers).build().unwrap();
        Self {
            client
        }
    }

    pub async fn get_patron(&self, discord_id: u64) -> Result<(Option<u64>, Option<u64>), RoError> {
        let mut link = Some("https://www.patreon.com/api/oauth2/v2/campaigns/3229705/members?include=currently_entitled_tiers,user&fields%5Buser%5D=social_connections".to_string());
        while link.is_some() {
            let res: Value = self.client.get(&link.unwrap()).send().await?.json::<Value>().await?;
            let info = res["included"].as_array().unwrap();
            let users = res["data"].as_array().unwrap();
            for user in info {
                if user["type"].as_str().unwrap().eq("user") {
                    let disc = user["attributes"]["social_connections"]["discord"].as_u64();
                    if let Some(disc) = disc {
                        if disc == discord_id {
                            let patreon_id = user["id"].as_u64().unwrap();
                            for u in users {
                                if u["relationships"]["user"]["data"]["id"].as_u64().unwrap() == patreon_id {
                                    let tiers = u["relationships"]["currently_entitled_tiers"]["data"].as_array().unwrap();
                                    if !tiers.is_empty() {
                                        return Ok((Some(patreon_id), tiers[0]["id"].as_u64()))
                                    }
                                    return Ok((Some(patreon_id), None));
                                }
                            }
                        }
                    }
                }
            }
            link = res["links"]["next"].as_str().map(|s| s.to_owned());
        }
        Ok((None, None))
    }
}