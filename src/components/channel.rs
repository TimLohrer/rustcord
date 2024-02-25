use leptos::*;
use wasm_bindgen::prelude::*;

use rustcord_lib::discord::{Discord, Channel};
use crate::app::AppState;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn Channel(
    state: ReadSignal<AppState>,
    set_state: WriteSignal<AppState>,
    discord: ReadSignal<Discord>,
    set_discord: WriteSignal<Discord>,
    channel: Channel
) -> impl IntoView {

    let switch_channel = move |channel_id: String| {
        let mut state = state.get();
        state.active_channel_id = channel_id;
        set_state.set(state);
    };
    
    let channel_id = channel.id.clone();

    view! {
       <div class={format!("channel {}", if state.get().active_channel_id == channel.id {"active"} else {""})} on:click={move |_| switch_channel(channel_id.clone())}>
            <p>{&channel.name}</p>
       </div>

       <style>"
       .channel > p {
            overflow: hidden;
        }
    
        .channel:hover, .channel.active {
            background-color: var(--discord-dark);
            color: var(--text-primary);
        }

        .channel {
            display: flex;
            align-items: center;
            text-align: left;
            padding: 3px;
            height: 35px;
            border-radius: 5px;
            padding: 2.5px 2.5px 2.5px 10px;
            cursor: pointer;
            }
       "</style>
    }
}