use leptos::*;
use rustcord_lib::data::message::message::Message;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use rustcord_lib::data::discord::app_data::AppData;
use rustcord_lib::data::channel::channel::Channel;
use crate::app::AppState;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GetMessagesArgs<'a> {
    token: &'a str,
    #[serde(rename = "channelId")]
    channel_id: &'a str,
}

#[component]
pub fn Channel(
    state: ReadSignal<AppState>,
    set_state: WriteSignal<AppState>,
    app_data: ReadSignal<AppData>,
    set_app_data: WriteSignal<AppData>,
    channel: Channel
) -> impl IntoView {

    let fetch_messages = move |channel_id: String| {
        spawn_local(async move {
            let mut app_data = app_data.get();
            let get_messages_args = to_value(&GetMessagesArgs { token: &app_data.token, channel_id: &channel_id }).unwrap();
            let mut messages: Vec<Message> = serde_wasm_bindgen::from_value(invoke("get_discord_messages", get_messages_args).await).unwrap();

            messages.reverse();

            app_data.message_cache.insert(channel_id.clone(), messages.clone());

            set_app_data.set(app_data.clone());
            // logging::log!("Fetched messages: {:?}", &messages);
        });
    };
    
    let switch_channel = move |channel_id: String| {
        let mut state = state.get();
        state.active_channel_id = channel_id.clone();

        set_state.set(state);
        
        if !app_data.clone().get().message_cache.contains_key(&channel_id) {
            fetch_messages(channel_id.clone());
        }
    };
    
    let channel_id = channel.id.clone();

    view! {
        <div class={format!("channel {}", if state.get().active_channel_id == channel.id {"active"} else {""})} on:click={move |_| switch_channel(channel_id.clone())}>
            {match channel.clone().r#type {
                0 => view! { <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" class="textChannelIcon" viewBox="0 0 24 24"><path fill="currentColor" fill-rule="evenodd" d="M10.99 3.16A1 1 0 1 0 9 2.84L8.15 8H4a1 1 0 0 0 0 2h3.82l-.67 4H3a1 1 0 1 0 0 2h3.82l-.8 4.84a1 1 0 0 0 1.97.32L8.85 16h4.97l-.8 4.84a1 1 0 0 0 1.97.32l.86-5.16H20a1 1 0 1 0 0-2h-3.82l.67-4H21a1 1 0 1 0 0-2h-3.82l.8-4.84a1 1 0 1 0-1.97-.32L15.15 8h-4.97l.8-4.84ZM14.15 14l.67-4H9.85l-.67 4h4.97Z" clip-rule="evenodd" class=""></path></svg> }.into_view(),
                2 => view! { <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path fill="currentColor" d="M12 3a1 1 0 0 0-1-1h-.06a1 1 0 0 0-.74.32L5.92 7H3a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h2.92l4.28 4.68a1 1 0 0 0 .74.32H11a1 1 0 0 0 1-1V3ZM15.1 20.75c-.58.14-1.1-.33-1.1-.92v-.03c0-.5.37-.92.85-1.05a7 7 0 0 0 0-13.5A1.11 1.11 0 0 1 14 4.2v-.03c0-.6.52-1.06 1.1-.92a9 9 0 0 1 0 17.5Z" class=""></path><path fill="currentColor" d="M15.16 16.51c-.57.28-1.16-.2-1.16-.83v-.14c0-.43.28-.8.63-1.02a3 3 0 0 0 0-5.04c-.35-.23-.63-.6-.63-1.02v-.14c0-.63.59-1.1 1.16-.83a5 5 0 0 1 0 9.02Z" class=""></path></svg> }.into_view(),
                5 => view! { <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24"><path fill="currentColor" fill-rule="evenodd" d="M19.56 2a3 3 0 0 0-2.46 1.28 3.85 3.85 0 0 1-1.86 1.42l-8.9 3.18a.5.5 0 0 0-.34.47v10.09a3 3 0 0 0 2.27 2.9l.62.16c1.57.4 3.15-.56 3.55-2.12a.92.92 0 0 1 1.23-.63l2.36.94c.42.27.79.62 1.07 1.03A3 3 0 0 0 19.56 22h.94c.83 0 1.5-.67 1.5-1.5v-17c0-.83-.67-1.5-1.5-1.5h-.94Zm-8.53 15.8L8 16.7v1.73a1 1 0 0 0 .76.97l.62.15c.5.13 1-.17 1.12-.67.1-.41.29-.78.53-1.1Z" clip-rule="evenodd" class=""></path><path fill="currentColor" d="M2 10c0-1.1.9-2 2-2h.5c.28 0 .5.22.5.5v7a.5.5 0 0 1-.5.5H4a2 2 0 0 1-2-2v-4Z" class=""></path></svg> }.into_view(),
                13 => view! { <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24"><path fill="currentColor" d="M19.61 18.25a1.08 1.08 0 0 1-.07-1.33 9 9 0 1 0-15.07 0c.26.42.25.97-.08 1.33l-.02.02c-.41.44-1.12.43-1.46-.07a11 11 0 1 1 18.17 0c-.33.5-1.04.51-1.45.07l-.02-.02Z" class=""></path><path fill="currentColor" d="M16.83 15.23c.43.47 1.18.42 1.45-.14a7 7 0 1 0-12.57 0c.28.56 1.03.6 1.46.14l.05-.06c.3-.33.35-.81.17-1.23A4.98 4.98 0 0 1 12 7a5 5 0 0 1 4.6 6.94c-.17.42-.13.9.18 1.23l.05.06Z" class=""></path><path fill="currentColor" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0ZM6.33 20.03c-.25.72.12 1.5.8 1.84a10.96 10.96 0 0 0 9.73 0 1.52 1.52 0 0 0 .8-1.84 6 6 0 0 0-11.33 0Z" class=""></path></svg> }.into_view(),
                15 => view! { <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18.91 12.98a5.45 5.45 0 0 1 2.18 6.2c-.1.33-.09.68.1.96l.83 1.32a1 1 0 0 1-.84 1.54h-5.5A5.6 5.6 0 0 1 10 17.5a5.6 5.6 0 0 1 5.68-5.5c1.2 0 2.32.36 3.23.98Z" class=""></path><path fill="currentColor" d="M19.24 10.86c.32.16.72-.02.74-.38L20 10c0-4.42-4.03-8-9-8s-9 3.58-9 8c0 1.5.47 2.91 1.28 4.11.14.21.12.49-.06.67l-1.51 1.51A1 1 0 0 0 2.4 18h5.1a.5.5 0 0 0 .49-.5c0-4.2 3.5-7.5 7.68-7.5 1.28 0 2.5.3 3.56.86Z"></path></svg> }.into_view(),
                _ => view! {<div></div>}.into_view()
            }}
            <p>{&channel.name}</p>
        </div>

        <style>"
        .channel {
            display: flex;
            flex-direction: row;
            align-items: center;
            text-align: left;
            height: 30px;
            border-radius: 5px;
            padding: 2.5px 2.5px 2.5px 10px;
            margin-bottom: 2.5px;
            cursor: pointer;
        }

        .channel:hover, .channel.active {
            background-color: var(--discord-dark);
            color: var(--primary-text);
        }

        .channel:hover > svg, .channel.active > svg {
            fill: var(--primary-text);
        }

        .channel > svg {
            width: 20px;
            height: 20px;
            margin-right: 5px;
            fill: var(--secondary-text);
        }

        .channel > p {
            font-size: 15px;
            font-weight: 500;
            white-space: nowrap;
            overflow: hidden;
        }
        "</style>
    }
}