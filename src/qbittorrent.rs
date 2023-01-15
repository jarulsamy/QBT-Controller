use reqwest::blocking::Response;
use serde_derive::Deserialize;
use serde_json::Value;

use std::fmt::Display;

#[derive(Deserialize, Clone, Debug)]
#[allow(unused)]
pub struct GlobalInfo {
    connection_status: String,
    dht_nodes: u64,
    dl_info_data: u64,
    dl_info_speed: u64,
    dl_rate_limit: u64,
    up_info_data: u64,
    up_info_speed: u64,
    up_rate_limit: u64,
    paused: Option<usize>,
    resumed: Option<usize>,
}

impl Default for GlobalInfo {
    fn default() -> Self {
        GlobalInfo {
            connection_status: "Unknown".to_string(),
            dht_nodes: 0,
            dl_info_data: 0,
            dl_info_speed: 0,
            dl_rate_limit: 0,
            up_info_data: 0,
            up_info_speed: 0,
            up_rate_limit: 0,
            paused: None,
            resumed: None,
        }
    }
}

impl Display for GlobalInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let paused = self.paused.unwrap_or(0);
        let resumed = self.resumed.unwrap_or(0);
        write!(
            f,
            "Download: {} Kb/s\nUpload Rate: {} Kb/s\nPaused: {}\nResumed: {}\n",
            self.dl_info_speed / 1000,
            self.up_info_speed / 1000,
            paused,
            resumed,
        )
    }
}

#[derive(Clone, Debug)]
#[allow(unused)]
pub struct QbtHost {
    pub name: String,
    pub base_url: String,
    base_api_url: String,
    username: Option<String>,
    password: Option<String>,
}

impl Default for QbtHost {
    fn default() -> Self {
        Self::new(
            "Localhost".to_string(),
            "http://localhost:8080".to_string(),
            None,
            None,
        )
    }
}

impl QbtHost {
    pub fn new(
        name: String,
        base_url: String,
        username: Option<String>,
        password: Option<String>,
    ) -> Self {
        let base_api_url = base_url.clone() + "/api/v2/";
        let host = Self {
            name,
            base_url,
            base_api_url,
            username,
            password,
        };

        return host;
    }

    pub fn get_info(&self) -> Result<GlobalInfo, reqwest::Error> {
        let global_info_endpoint = self.base_api_url.clone() + "transfer/info";
        let mut global_info: GlobalInfo = reqwest::blocking::get(global_info_endpoint)?.json()?;

        let resumed_torrents_endpoint = self.base_api_url.clone() + "torrents/info?filter=resumed";
        let binding = reqwest::blocking::get(resumed_torrents_endpoint)?.json::<Value>()?;
        let resumed_torrents_count = match binding.as_array() {
            Some(x) => x.len(),
            None => 0,
        };

        let paused_torrents_endpoint = self.base_api_url.clone() + "torrents/info?filter=paused";
        let binding = reqwest::blocking::get(paused_torrents_endpoint)?.json::<Value>()?;
        let paused_torrents_count = match binding.as_array() {
            Some(x) => x.len(),
            None => 0,
        };

        global_info.paused = Some(paused_torrents_count);
        global_info.resumed = Some(resumed_torrents_count);

        Ok(global_info)
    }

    pub fn pause(&self) -> Result<Response, reqwest::Error> {
        let endpoint = self.base_api_url.clone() + "torrents/pause?hashes=all";
        reqwest::blocking::get(endpoint)
    }

    pub fn resume(&self) -> Result<Response, reqwest::Error> {
        let endpoint = self.base_api_url.clone() + "torrents/resume?hashes=all";
        reqwest::blocking::get(endpoint)
    }
}
