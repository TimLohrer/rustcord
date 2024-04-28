use std::time::Duration;

use leptos::*;
use rustcord_lib::data::discord::app_data::AppData;
use rustcord_lib::data::discord::settings::Settings;
use rustcord_lib::data::discord::user::User;
use rustcord_lib::data::guild::guild_minimal::GuildMinimal;
use wasm_bindgen::prelude::*;
use crate::components::screens::login::LoginScreen;
use crate::components::screens::discord::DiscordScreen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub screen: Screen,
    pub active_guild_id: String,
    pub active_channel_id: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            screen: Screen::LOADING,
            active_guild_id: String::from("HOME"),
            active_channel_id: String::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Screen {
    LOADING,
    DISCORD,
    LOGIN,
}

const OFFLINE_MODE: bool = false;

#[component]
pub fn App() -> impl IntoView {
    let (state, set_state) = create_signal(AppState::new());
    let (app_data, set_app_data) = create_signal(AppData::new());

    if OFFLINE_MODE {
        let mut debug_state = state.clone().get();
        debug_state.screen = Screen::DISCORD;
        set_state.set(debug_state);

        let mut debug_discord = app_data.clone().get();
        debug_discord.token = String::from("DEV_TOKEN");
        debug_discord.settings = Settings::new();
        debug_discord.user = User {
            id: String::from("86234872634876783"),
            email: Some(String::from("dev@example.com")),
            global_name: Some(String::from("debug_user")),
            username: String::from("DEBUG USER"),
            discriminator: String::from("0"),
            bio: String::from("DEBUG BIO"),
            locale: String::from("en-US"),
            verified: true,
            mfa_enabled: false,
            flags: 73248,
            premium_type: 0,
            public_flags: 3294234,
            bot: None,
            nsfw_allowed: Some(true),
            accent_color: None,
            purchased_flags: None,
            authenticator_types: None,
            avatar: None,
            avatar_decoration_data: None,
            banner: None,
            banner_color: None,
            phone: None,
            linked_users: None,
        };
        debug_discord.guilds_minimal = vec![
            GuildMinimal {
                id: String::from("987346598763457"),
                name: String::from("DEBUG SERVER 1"),
                owner: true,
                icon: None,
                permissions: String::new(),
                features: vec![]
            }
        ];
        // debug_discord.guilds = vec![
        //     Guild {
        //         id: String::from("987346598763457"),
        //         name: String::from("DEBUG SERVER 1"),
        //         channels: Some(vec![
        //             Channel {
        //                 id: String::from("1234567890"),
        //                 guild_id: String::from("987346598763457"),
        //                 r#type: 4,
        //                 name: String::from("cool channels"),
        //                 position: 1,
        //                 parent_id: None,

        //             },
        //         ]),
        //         ..Default::default()
        //     }
        // ];
        set_app_data.set(debug_discord);
    }

    set_timeout(move || {
        if OFFLINE_MODE { return; } 
        let mut state = state.get();
        state.screen = Screen::LOGIN;
        set_state.set(state);
    }, Duration::from_secs(1));
    
    view! {
        <main class="app">
            {move || match state.get().screen {
                Screen::LOADING => view! { <p>"Loading..."</p> }.into_view(),
                Screen::DISCORD => view! { <DiscordScreen state=state set_state=set_state app_data=app_data set_app_data=set_app_data /> }.into_view(),
                Screen::LOGIN => view! { <LoginScreen state=state set_state=set_state app_data=app_data set_app_data=set_app_data /> }.into_view(),
            }}
        </main>

        <style>"
            * {
                margin: 0;
                padding: 0;
                box-sizing: border-box;
            }

            .app {
                height: 100vh;
                width: 100vw;
                background-color: var(--primary-background);
                color: var(--primary-text);
            }
        "</style>
    }
}
