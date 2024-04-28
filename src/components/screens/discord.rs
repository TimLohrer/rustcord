use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use rustcord_lib::data::discord::app_data::AppData;

use crate::components::guild::Guild;
use crate::components::guilds_sidebar::GuildsSidebar;
use crate::app::AppState;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GetGuildsArgs<'a> {
    token: &'a str
}

#[component]
pub fn DiscordScreen(
    state: ReadSignal<AppState>,
    set_state: WriteSignal<AppState>,
    app_data: ReadSignal<AppData>,
    set_app_data: WriteSignal<AppData>,
) -> impl IntoView {
    let fetch_guilds = move || {
        spawn_local(async move {
            let mut app_data = app_data.get();
            
            let get_guilds_args = to_value(&GetGuildsArgs { token: &app_data.token }).unwrap();
            app_data.guilds_minimal = invoke("get_discord_guilds", get_guilds_args).await.into_serde().unwrap();
            set_app_data.set(app_data.clone());
            logging::log!("Fetched guilds: {:?}", &app_data.guilds_minimal);
        });

    };

    if app_data.clone().get().guilds_minimal.is_empty() {
        logging::log!("Fetching guilds...");
        fetch_guilds();
    }
    
    view! {
        <main class="discord-screen">
            {move || if state.get().active_guild_id == "HOME" {
                let _ = app_data.get();
                view! {
                    <GuildsSidebar state=state set_state=set_state app_data=app_data.clone() set_app_data=set_app_data />
                    <p>HOME</p>
                }.into_view()
            } else {
                let _ = app_data.get();
                view! {
                    <GuildsSidebar state=state set_state=set_state app_data=app_data set_app_data=set_app_data />
                    <Guild state=state set_state=set_state app_data=app_data set_app_data=set_app_data />
                }.into_view()
            }}
        </main>

        <style>"
            .discord-screen {
                display: flex;
            }
        "</style>
    }
}
