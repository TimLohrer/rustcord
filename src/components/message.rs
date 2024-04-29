use leptos::*;
use rustcord_lib::data::message::message::Message;
use wasm_bindgen::prelude::*;

use rustcord_lib::data::discord::app_data::AppData;
use crate::app::AppState;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn Message(
    state: ReadSignal<AppState>,
    set_state: WriteSignal<AppState>,
    app_data: ReadSignal<AppData>,
    set_app_data: WriteSignal<AppData>,
    message: Message
) -> impl IntoView {

    view! {
        <div class="message">
            <div class="authorAvatarContainer">
                <img class="authorAvatar" src={if message.clone().author.avatar.is_some() { format!("https://cdn.discordapp.com/avatars/{}/{}.webp?size=128", message.clone().author.id, message.clone().author.avatar.unwrap()) } else { String::from("../../public/assets/discord/default_pfp.png") }} alt="user pfp" />
            </div>
            <div class="body">
                <div class="header">
                    <p class="author">{if message.clone().author.global_name.is_some() { message.clone().author.global_name.unwrap() } else { message.clone().author.username }}</p>
                    <p class="timestamp">{&message.timestamp}</p>
                </div>
                <p class="content">{&message.content}</p>
            </div>
        </div>

        <style>"
            .message {
                display: flex;
                flex-direction: row;
                justify-content: flex-start;
                text-align: left;
                border-radius: 5px;
                padding: 5px 5px 5px 5px;
                margin-bottom: 10px;
            }
        
            .message:hover {
                background-color: var(--secondary-background);
            }
        
            .message > .authorAvatarContainer {
                display: flex;
                flex-direction: column;
                justify-content: flex-start;
                align-items: center;
                margin-right: 10px;
            }
        
            .message > .authorAvatarContainer > img.authorAvatar {
                width: 45px;
                height: 45px;
                border-radius: 50%;
                margin-right: 10px;
            }
        
            .message > .body > .header {
                display: flex;
                flex-direction: row;
                justify-content: flex-start;
                align-items: center;
                height: 20px;
                margin-bottom: 5px;
            }
        
            .message > .body > .header > .author {
                font-size: 17.5px;
                font-weight: 500;
                margin-right: 15px;
            }
        
            .message > .body > .header > .timestamp {
                font-size: 13.5px;
                font-weight: 400;
                color: var(--secondary-text);
            }
        
            .message > .body > .content {
                font-size: 15px;
                font-weight: 400;
            }
        "</style>
    }
}