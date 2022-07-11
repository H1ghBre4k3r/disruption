use reqwest::Response;
use serde::Serialize;

pub struct RestClient {
    bot_token: String,
    base_url: String,
    client: reqwest::Client,
}

/// Client for the Discord REST API. (basically a wrapper around reqwest)
impl RestClient {
    pub fn new(bot_token: &String, api_version: u8) -> Self {
        Self {
            base_url: format!("https://discord.com/api/v{}/", api_version),
            bot_token: bot_token.clone(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn post<T: Serialize + ?Sized>(
        &self,
        uri: &String,
        content: &T,
    ) -> Result<Response, reqwest::Error> {
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
    }
}
