use leptos::*;
use wasm_bindgen::prelude::*;

use rustcord_lib::discord::Discord;

use crate::AppState;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn CurrentUserPanel(
    state: ReadSignal<AppState>,
    set_state: WriteSignal<AppState>,
    discord: ReadSignal<Discord>,
) -> impl IntoView {
    view! {
        <div class={"current_user_panel"}>
            
        </div>

        <style>"
            .guilds_sidebar {
                display: flex;
                background-color: var(--discord-dark);
                flex-direction: column;
                align-items: center;
                width: 75px;
                height: 100vh;
                padding-top: 10px;
                overflow-y: auto;
            }

            .guilds_sidebar::-webkit-scrollbar {
                width: 0px;
            }

            .guilds_sidebar > .guildButton {
                display: flex;
                align-items: center;
                justify-content: center;
                width: 55px;
                height: 55px;
                border-radius: 100%;
                margin-top: 10px;
                cursor: pointer;
                transition: 0.2s;
            }

            .guilds_sidebar > .guildButton.no-icon {
                background-color: var(--secondary-background);
            }

            .guilds_sidebar > .guildButton:hover, .guilds_sidebar > .guildButton.active {
                border-radius: 35%;
                transition: 0.2s;
            }

            .guilds_sidebar > .guildButton img {
                object-fit: cover;
                border-radius: 100%;
                height: 55px;
                width: 55px;
                transition: 0.2s;
            }

            .guilds_sidebar > .guildButton img:hover, .guilds_sidebar > .guildButton.active img {
                border-radius: 35%;
                transition: 0.2s;
            }

            .guilds_sidebar > .guildButton p {
                font-size: 20px;
                font-weight: w600;
                color: var(--text-primary);
            }

            .guilds_sidebar > .homeButton {
                display: flex;
                align-items: center;
                justify-content: center;
                width: 55px;
                height: 55px;
                border-radius: 100%;
                padding: 10px;
                background-color: var(--secondary-background);
                cursor: pointer;
                transition: 0.2s;
            }

            .guilds_sidebar > .homeButton img {
                height: 25px;
                object-fit: contain;
            }

            .guilds_sidebar > .homeButton:hover, .guilds_sidebar > .homeButton.active {
                border-radius: 35%;
                background-color: var(--blurple);
                transition: 0.2s;
            }
        "</style>
    }.into_view()
}
