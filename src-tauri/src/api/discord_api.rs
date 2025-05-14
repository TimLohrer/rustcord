use reqwest;
use serde_json::Value;

const API_VERSION: &str = "v10";

#[derive(serde::Deserialize)]
struct GatewayUrlResponse {
    url: String
}

pub struct DiscordApi {}

impl DiscordApi {
    fn get_api_base() -> String {
        format!("https://discord.com/api/{}", API_VERSION)
    }

    pub fn get_authorization_header(token: String, bot_user: bool) -> String {
        format!("{}{}", if bot_user {"Bot "} else {""}, token)
    }

    pub async fn login(token: String) -> Result<Value, reqwest::Error>{
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/users/@me", DiscordApi::get_api_base()))
            .header("Authorization", token)
            .send().await?
            .error_for_status()?;
        let json = res.json().await.unwrap();
        Ok(json)
    }

    pub async fn get_gateway_url() -> Result<String, reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/gateway", DiscordApi::get_api_base()))
            .send().await?
            .error_for_status()?;
        let json = res.json::<GatewayUrlResponse>().await.unwrap();
        Ok(json.url)
    }
    
    pub async fn get_discord_settings(token: String) -> Result<Value, reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/users/@me/settings", DiscordApi::get_api_base()))
            .header("Authorization", token)
            .send().await?
            .error_for_status()?;
        let json = res.json().await.unwrap();
        Ok(json)
    }
    
    pub async fn get_dm_channels(token: String) -> Result<Value, reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/users/@me/channels", DiscordApi::get_api_base()))
            .header("Authorization", token)
            .send()
            .await?
            .error_for_status()?;
        let json = res.json().await.unwrap();
        Ok(json)
    }
    
    pub async fn get_guilds(token: String) -> Result<Value, reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/users/@me/guilds", DiscordApi::get_api_base()))
            .header("Authorization", token)
            .send()
            .await?
            .error_for_status()?;
        let json = res.json().await.unwrap();
        Ok(json)
    }
    
    pub async fn get_guild(token: String, guild_id: String) -> Result<Value, reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/guilds/{}", DiscordApi::get_api_base(), guild_id))
            .header("Authorization", token)
            .send()
            .await?
            .error_for_status()?;
        let json = res.json().await.unwrap();
        Ok(json)
    }
    
    pub async fn get_guild_channels(token: String, guild_id: String) -> Result<Value, reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/guilds/{}/channels", DiscordApi::get_api_base(), guild_id))
            .header("Authorization", token)
            .send()
            .await?
            .error_for_status()?;
        let json = res.json::<Value>().await.unwrap();
        Ok(json)
    }

    pub async fn get_messages(token: String, channel_id: String) -> Result<Value, reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/channels/{}/messages?limit=50", DiscordApi::get_api_base(), channel_id))
            .header("Authorization", token)
            .send()
            .await?
            .error_for_status()?;
        let json = res.json().await.unwrap();
        Ok(json)
    }
}