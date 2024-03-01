use leptos::*;
use wasm_bindgen::prelude::*;

use rustcord_lib::discord::{Discord, Channel};
use crate::app::AppState;
use crate::components::channel::Channel as ChannelComponent;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn CategoryChannel(
    state: ReadSignal<AppState>,
    set_state: WriteSignal<AppState>,
    discord: ReadSignal<Discord>,
    set_discord: WriteSignal<Discord>,
    channel: Channel
) -> impl IntoView {
    let (show_children, set_show_children) = create_signal(true);

    let toggle_show_children = move || {
        logging::log!("Toggling show children");
        set_show_children.set(!show_children.get());
    };

    view! {
        <div class={"categoryChannel"} on:click={move |_| toggle_show_children()}>
            <div class="categoryChannelTitle">
                <svg class={move || format!("{}", if show_children.get() {"open"} else {"closed"})} xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5.3 9.3a1 1 0 0 1 1.4 0l5.3 5.29 5.3-5.3a1 1 0 1 1 1.4 1.42l-6 6a1 1 0 0 1-1.4 0l-6-6a1 1 0 0 1 0-1.42Z"></path></svg>
                <p>{channel.name}</p>
            </div>
            {move || if show_children.get() {
                {channel.children.as_ref().unwrap().into_iter().map(|child| {
                    view! {
                        <ChannelComponent state=state set_state=set_state discord=discord set_discord=set_discord channel=child.clone() />
                    }
                })}.collect::<Vec<_>>().into_view()
            } else {
                view! { <div></div> }.into_view()
            }}   
       </div>
       <style>"
        .categoryChannel {
            display: flex;
            flex-direction: column;
            margin: 0 10px 15px 0;
            cursor: pointer;
        }

        .categoryChannel > .categoryChannelTitle {
            display: flex;
        }

        .categoryChannel > .categoryChannelTitle > p {
            font-size: 14px;
            font-weight: bold;
            width: 100%;
            color: var(--secondary-text);
            transition: 0.2s;
        }
        
        .categoryChannel > .categoryChannelTitle > p:hover {
            color: var(--primary-text);
        }

        .categoryChannel > .categoryChannelTitle > svg {
            width: 20px;
            height: 20px;
            margin-right: 5px;
            fill: var(--secondary-text);
            transition: 0.2s;
        }

        .categoryChannel:hover > .categoryChannelTitle > svg {
            fill: var(--primary-text);
        }
        
        .categoryChannel > .categoryChannelTitle > svg.open {
            rotate: 0deg;
            transition: 0.2s;
        }

        .categoryChannel > .categoryChannelTitle > svg.closed {
            rotate: -90deg;
            transition: 0.2s;
        }
    
        .categoryChannel > .channel {
            margin-left: 12.5px;
        }
       "</style>
    }
}
