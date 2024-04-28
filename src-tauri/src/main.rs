// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod api;

use reqwest::Error;
use rustcord_lib::data::{channel::channel::Channel, discord::{settings::Settings, user::User}, guild::{guild::Guild, guild_minimal::GuildMinimal}, message::message::Message};

use crate::api::discord_api::DiscordApi;

pub const VERBOSE: bool = true;
pub const BOT_USER: bool = false;

#[tauri::command]
async fn discord_login(token: String) -> Result<User, String> {
    if VERBOSE {
        println!("Called discord_login: {:?}", token);
    }
    let result: Result<User, Error> = DiscordApi::login(DiscordApi::get_authorization_header(token, BOT_USER)).await;
    match result {
        Ok(user) => Ok(user),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
async fn get_discord_settings(token: String) -> Result<Settings, String> {
    if VERBOSE {
        println!("Called get_discord_settings: {:?}", token);
    }
    let result: Result<Settings, Error> = DiscordApi::get_discord_settings(DiscordApi::get_authorization_header(token, BOT_USER)).await;
    match result {
        Ok(settings) => Ok(settings),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
async fn get_discord_guilds(token: String) -> Result<Vec<GuildMinimal>, String> {
    if VERBOSE {
        println!("Called get_discord_guilds: {:?}", token);
    }
    let result: Result<Vec<GuildMinimal>, Error> = DiscordApi::get_guilds(DiscordApi::get_authorization_header(token, BOT_USER)).await;
    match result {
        Ok(guilds) => Ok(guilds),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
async fn get_discord_guild(token: String, guild_id: String) -> Result<Guild, String> {
    if VERBOSE {
        println!("Called get_discord_guild: {:?} {:?}", token, guild_id);
    }
    let result: Result<Guild, Error> = DiscordApi::get_guild(DiscordApi::get_authorization_header(token, BOT_USER), guild_id).await;
    match result {
        Ok(guild) => Ok(guild),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
async fn get_discord_guild_channels(token: String, guild_id: String) -> Result<Vec<Channel>, String> {
    if VERBOSE {
        println!("Called get_discord_guild_channels: {:?} {:?}", token, guild_id);
    }
    let result: Result<Vec<Channel>, Error> = DiscordApi::get_guild_channels(DiscordApi::get_authorization_header(token, BOT_USER), guild_id).await;
    match result {
        Ok(channels) => Ok(channels),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
async fn get_discord_messages(token: String, channel_id: String) -> Result<Vec<Message>, String> {
    if VERBOSE {
        println!("Called get_discord_messages: {:?} {:?}", token, channel_id);
    }
    let result: Result<Vec<Message>, Error> = DiscordApi::get_messages(DiscordApi::get_authorization_header(token, BOT_USER), channel_id).await;
    match result {
        Ok(messages) => Ok(messages),
        Err(e) => Err(e.to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            discord_login,
            get_discord_settings,
            get_discord_guilds,
            get_discord_guild,
            get_discord_guild_channels,
            get_discord_messages
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
