use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use crate::components::guild::Guild;
use crate::components::guilds_sidebar::GuildsSidebar;
use crate::app::AppState;
use rustcord_lib::discord::Discord;

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
    discord: ReadSignal<Discord>,
    set_discord: WriteSignal<Discord>,
) -> impl IntoView {
    let fetch_guilds = move || {
        spawn_local(async move {
            let mut discord = discord.get();
            
            let get_guilds_args = to_value(&GetGuildsArgs { token: &discord.token }).unwrap();
            discord.guilds_minimal = invoke("get_discord_guilds", get_guilds_args).await.into_serde().unwrap();
            set_discord.set(discord.clone());
            logging::log!("Fetched guilds: {:?}", &discord.guilds_minimal);
        });

    };

    if discord.clone().get().guilds_minimal.is_empty() {
        logging::log!("Fetching guilds...");
        fetch_guilds();
    }
    
    view! {
        <main class="discord-screen">
            {move || if state.get().active_guild_id == "HOME" {
                let _ = discord.get();
                view! {
                    <GuildsSidebar state=state set_state=set_state discord=discord.clone() set_discord=set_discord />
                    <p>HOME</p>
                }.into_view()
            } else {
                let _ = discord.get();
                view! {
                    <GuildsSidebar state=state set_state=set_state discord=discord set_discord=set_discord />
                    <Guild state=state set_state=set_state discord=discord set_discord=set_discord />
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
