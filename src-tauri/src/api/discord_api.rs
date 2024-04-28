use reqwest;
use rustcord_lib::data::{channel::channel::Channel, discord::{settings::Settings, user::User}, guild::{guild::Guild, guild_minimal::GuildMinimal}};

use crate::VERBOSE;

const API_VERSION: &str = "v9";

pub struct DiscordApi {}

impl DiscordApi {
    fn get_api_base() -> String {
        format!("https://discord.com/api/{}", API_VERSION)
    }

    pub fn get_authorization_header(token: String, bot_user: bool) -> String {
        let auth_header = format!("{}{}", if bot_user {"Bot "} else {""}, token);
        if VERBOSE {
            println!("Generated auth header: {:?}", auth_header)
        }
        auth_header
    }

    pub async fn login(token: String) -> Result<User, reqwest::Error>{
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/users/@me", DiscordApi::get_api_base()))
            .header("Authorization", token)
            .send().await?
            .error_for_status()?;
        let json = res.json::<User>().await.unwrap();
        Ok(json)
    }
    
    pub async fn get_discord_settings(token: String) -> Result<Settings, reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/users/@me/settings", DiscordApi::get_api_base()))
            .header("Authorization", token)
            .send().await?
            .error_for_status()?;
        let json = res.json::<Settings>().await.unwrap();
        Ok(json)
    }

    pub async fn get_guilds(token: String) -> Result<Vec<GuildMinimal>, reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/users/@me/guilds", DiscordApi::get_api_base()))
            .header("Authorization", token)
            .send()
            .await?
            .error_for_status()?;
        let json = res.json::<Vec<GuildMinimal>>().await.unwrap();
        Ok(json)
    }
    
    pub async fn get_guild(token: String, guild_id: String) -> Result<Guild, reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/guilds/{}", DiscordApi::get_api_base(), guild_id))
            .header("Authorization", token)
            .send()
            .await?
            .error_for_status()?;
        let json = res.json::<Guild>().await.unwrap();
        Ok(json)
    }
    
    pub async fn get_guild_channels(token: String, guild_id: String) -> Result<Vec<Channel>, reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/guilds/{}/channels", DiscordApi::get_api_base(), guild_id))
            .header("Authorization", token)
            .send()
            .await?
            .error_for_status()?;
        let json = res.json::<Vec<Channel>>().await.unwrap();
        Ok(json)
    }
}