use leptos::{*};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use web_sys::SubmitEvent;

use crate::app::AppState;
use crate::Screen;
use rustcord_lib::discord::{Discord, Settings, User};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct LoginArgs<'a> {
    token: &'a str
}

#[derive(Serialize, Deserialize)]
struct SettingsArgs<'a> {
    token: &'a str
}

#[component]
pub fn LoginScreen(
    state: ReadSignal<AppState>,
    set_state: WriteSignal<AppState>,
    discord: ReadSignal<Discord>,
    set_discord: WriteSignal<Discord>,
) -> impl IntoView {
    let (token, set_token) = create_signal(String::new());

    let update_token = move |ev| {
        let v = event_target_value(&ev);
        set_token.set(v);
    };

    let login = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let mut state = state.get_untracked();
            let mut discord = discord.get_untracked();
            let token = token.get_untracked();
            if token.is_empty() {
                return;
            }
            
            let login_args = to_value(&LoginArgs { token: &token }).unwrap();
            let user: User = invoke("discord_login", login_args).await.into_serde().unwrap();
            if user.id.is_empty() {
                // User auth failed -> TODO: Show error message
                return;
            }

            let settings_args = to_value(&SettingsArgs { token: &token }).unwrap();
            let settings: Settings = invoke("get_discord_settings", settings_args).await.into_serde().unwrap();
            logging::log!("Settings: {:?}", &settings);
            
            discord.token = token;
            discord.settings = settings;
            discord.user = user;
            set_discord.set(discord.clone());
            logging::log!("Login success: {:?}", &discord.user.username);

            // update app state screen
            state.screen = Screen::DISCORD;
            set_state.set(state);
        });
    };

    view! {
        <main class="container">
            <form class="login_form" on:submit=login>
                <input
                    id="login-input"
                    placeholder="Enter your token..."
                    on:input=update_token
                />
                <button type="submit">Login</button>
            </form>
        </main>
    }
}
