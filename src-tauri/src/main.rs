// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod api;

use tauri::Manager;
use tokio::sync::Mutex;
use lazy_static::lazy_static;
use serde_json::Value;
use dotenv;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

use crate::api::discord_api::DiscordApi;

pub const VERBOSE: bool = true;
pub const BOT_USER: bool = false;

lazy_static! {
    static ref TOKEN: Mutex<String> = Mutex::new(String::new());
}

#[tauri::command]
async fn get_debug_token() -> Result<Option<String>, String> {
    if VERBOSE {
        println!("Called get_debug_token");
    }
    let token = TOKEN.lock().await;
    if token.is_empty() {
        Ok(None)
    } else {
        Ok(Some(token.clone()))
    }
} 

#[tauri::command]
async fn discord_token_login(token: String) -> Result<Value, String> {
    if VERBOSE {
        println!("Called discord_login");
    }
    
    DiscordApi::login(DiscordApi::get_authorization_header(token, BOT_USER)).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_discord_settings(token: String) -> Result<Value, String> {
    if VERBOSE {
        println!("Called get_discord_settings");
    }
    
    DiscordApi::get_discord_settings(DiscordApi::get_authorization_header(token, BOT_USER)).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_discord_guilds(token: String) -> Result<Value, String> {
    if VERBOSE {
        println!("Called get_discord_guilds");
    }
    
    DiscordApi::get_guilds(DiscordApi::get_authorization_header(token, BOT_USER)).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_discord_guild(token: String, guild_id: String) -> Result<Value, String> {
    if VERBOSE {
        println!("Called get_discord_guild: {:?} {:?}", token, guild_id);
    }
    
    DiscordApi::get_guild(DiscordApi::get_authorization_header(token, BOT_USER), guild_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_discord_guild_channels(token: String, guild_id: String) -> Result<Value, String> {
    if VERBOSE {
        println!("Called get_discord_guild_channels: {:?} {:?}", token, guild_id);
    }
    
    DiscordApi::get_guild_channels(DiscordApi::get_authorization_header(token, BOT_USER), guild_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_discord_messages(token: String, channel_id: String) -> Result<Value, String> {
    if VERBOSE {
        println!("Called get_discord_messages: {:?} {:?}", token, channel_id);
    }
    
    DiscordApi::get_messages(DiscordApi::get_authorization_header(token, BOT_USER), channel_id).await.map_err(|e| e.to_string())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, Some(16.0))
                .expect("Unsupported platform!");
            
            let discord_token = dotenv::var("DISCORD_TOKEN").unwrap_or_else(|_| String::new());
            if !discord_token.is_empty() {
                if VERBOSE {
                    println!("Found debug Discord token: {:?}", discord_token);
                }
                tokio::spawn(async move {
                    let mut token = TOKEN.lock().await;
                    *token = discord_token.clone();
                    drop(token);
                });
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_debug_token,
            discord_token_login,
            get_discord_settings,
            get_discord_guilds,
            get_discord_guild,
            get_discord_guild_channels,
            get_discord_messages
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
