use core::fmt::Debug;
use reqwest::Response;
use serde::Serialize;

use crate::error::RestResult;

#[derive(Clone)]
pub struct RestClient {
    bot_token: String,
    base_url: String,
    client: reqwest::Client,
}

/// Client for the Discord REST API. (basically a wrapper around reqwest)
impl RestClient {
    pub fn new(bot_token: &str, api_version: u8) -> Self {
        Self {
            base_url: format!("https://discord.com/api/v{api_version}/"),
            bot_token: bot_token.to_owned(),
            client: reqwest::Client::new(),
        }
    }

    /// Send a POST to the API.
    pub async fn post<T: Serialize + ?Sized>(
        &self,
        uri: &String,
        content: &T,
    ) -> RestResult<Response> {
        self.client
            .post(format!("{}/{}", self.base_url, uri))
            .header("Authorization", format!("Bot {}", self.bot_token))
            .header(
                "User-Agent",
                "DiscordBot (https://github.com/H1ghBre4k3r/disruption, 0.1.0)",
            )
            .json(content)
            .send()
            .await
            .map_err(Into::into)
    }

    /// Issue a GET request to the API.
    pub async fn get(&self, uri: &String) -> RestResult<Response> {
        self.client
            .get(format!("{}/{}", self.base_url, uri))
            .header("Authorization", format!("Bot {}", self.bot_token))
            .header(
                "User-Agent",
                "DiscordBot (https://github.com/H1ghBre4k3r/disruption, 0.1.0)",
            )
            .send()
            .await
            .map_err(Into::into)
    }
}

impl Debug for RestClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("RestClient {{ base_url: '{}' }}", self.base_url))
    }
}
