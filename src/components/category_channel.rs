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
            <p>{channel.name}</p>
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
            margin-top: 15px;
            cursor: pointer;
        }

        .categoryChannel > .channel {
            margin-left: 15px;
        }
       "</style>
    }
}
