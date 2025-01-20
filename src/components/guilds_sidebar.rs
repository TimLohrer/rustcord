use leptos::*;
use wasm_bindgen::prelude::*;

use rustcord_lib::data::discord::app_data::AppData;

use crate::AppState;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn GuildsSidebar(
    state: ReadSignal<AppState>,
    set_state: WriteSignal<AppState>,
    app_data: ReadSignal<AppData>,
    set_app_data: WriteSignal<AppData>,
) -> impl IntoView {
    let switch_guild = move |guild_id: String| {
        let mut state = state.get();
        state.active_guild_id = guild_id;
        state.active_channel_id = "".to_string();
        set_state.set(state);
    };

    let join_guild = move || {
        logging::log!("Join a guild...");
    };

    view! {
        <div class={"guilds_sidebar"}>
            <div on:click={move |_| switch_guild(String::from("HOME"))} class={format!("homeButton hasIndicatorHandle {}", if state.clone().get().active_guild_id == String::from("HOME") {"active"} else {""})}>
                // <div class={format!("indicatorHandle {}", if state.clone().get().active_guild_id == String::from("HOME") {"active"} else {""})} />
                <img src={"../../public/assets/discord/logo_white.png"} />
            </div>
            <div class={"seperator-wrapper"}>
            <span class={"seperator"} />
            </div>
            {
                app_data.clone().get().guilds_minimal.into_iter().map(|guild| {
                    let guild_id = guild.id.clone();
                    view! {
                        <div on:click={move |_| switch_guild(guild_id.clone())} class={format!("guildButton hasIndicatorHandle {} {}", if guild.clone().icon.is_none() {"no-icon"} else {""}, if state.get().active_guild_id == guild.id {"active"} else {""})}>
                        // <div class={format!("indicatorHandle {}", if state.clone().get().active_guild_id == guild.clone().id {"active"} else {""})} />
                        {if guild.icon.is_some() {
                            view! { <img src={format!("https://cdn.discordapp.com/icons/{}/{}.webp?size=128", &guild.id, guild.clone().icon.unwrap_or(String::new()))} /> }.into_view()
                        } else {
                            view! { <p>{guild.name.chars().nth(0).unwrap().to_uppercase().to_string()}</p> }.into_view()
                        }}
                        </div>
                    }
                }).collect::<Vec<_>>().into_view()
            }
            <div class={"seperator-wrapper"}>
                <span class={"seperator"} />
            </div>
            <div on:click={move |_| join_guild()} class={"actionButton hasIndicatorHandle"}>
                // <div class={format!("indicatorHandle {}", if state.clone().get().active_guild_id == String::from("HOME") {"active"} else {""})} />
                <svg class="circleIcon__428dd" aria-hidden="true" role="img" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
                    <path d="M13 5a1 1 0 1 0-2 0v6H5a1 1 0 1 0 0 2h6v6a1 1 0 1 0 2 0v-6h6a1 1 0 1 0 0-2h-6V5Z" class=""></path>
                </svg>
            </div>
        </div>

        <style>"
            .guilds_sidebar {
                display: flex;
                background-color: var(--discord-dark);
                flex-direction: column;
                align-items: center;
                width: 75px;
                height: 100vh;
                overflow-y: auto;
            }

            .guilds_sidebar::-webkit-scrollbar {
                width: 0px;
            }

            .seperator-wrapper {
                display: grid;
                place-item: center;
                align-items: center;
                justify-content: center;
                padding: 10px 0 10px 0;
            }
            
            .seperator {
                width: 40px;
                height: 3px;
                background-color: var(--primary-background);
                border-radius: 5px;
            }
            /*
            .guilds_sidebar > .hasIndicatorHandle > .indicatorHandle {
                position: absolute;
                display: flex;
                align-items: center;
                justify-self: center;
                place-self: center;
                background-color: var(--white);
                width: 10px;
                border-radius: 5px;
                left: -6px;
                transition: 0.2s ease;
            }
            
            .guilds_sidebar > .hasIndicatorHandle > .indicatorHandle.active {
                height: 40px;
                margin-top: 5px;
                left: -6px;
                transition: 0.2s ease;
            }
            
            .guilds_sidebar > .hasIndicatorHandle > .indicatorHandle.active:hover {
                height: 40px;
                left: -6px;
                transition: 0.2s ease;
            }
            
            .guilds_sidebar > .hasIndicatorHandle > .indicatorHandle.notification {
                height: 10px;
                left: -6px;
                transition: 0.2s ease;
            }
            
            .guilds_sidebar > .hasIndicatorHandle:hover > .indicatorHandle.active {
                height: 40px;
                left: -6px;
                transition: 0.2s ease;
            }
            
            .guilds_sidebar > .hasIndicatorHandle:hover > .indicatorHandle {
                height: 22.5px;
                left: -6px;
                transition: 0.2s ease;
            }
            */

            .guilds_sidebar > .guildButton {
                display: grid;
                place-item: center;
                align-items: center;
                justify-content: center;
                width: 55px;
                height: 55px;
                border-radius: 100%;
                margin-bottom: 10px;
                cursor: pointer;
                transition: 0.3s ease;
            }

            .guilds_sidebar > .guildButton.no-icon {
                background-color: var(--secondary-background);
            }

            .guilds_sidebar > .guildButton:hover, .guilds_sidebar > .guildButton.active {
                border-radius: 30%;
                transition: 0.3s ease;
            }

            .guilds_sidebar > .guildButton img {
                object-fit: cover;
                border-radius: 100%;
                height: 55px;
                width: 55px;
                transition: 0.3s ease;
            }

            .guilds_sidebar > .guildButton img:hover, .guilds_sidebar > .guildButton.active img {
                border-radius: 30%;
                transition: 0.3s ease;
            }

            .guilds_sidebar > .guildButton p {
                font-size: 20px;
                font-weight: w600;
                color: var(--primary-text);
            }

            .guilds_sidebar > .homeButton {
                display: grid;
                place-item: center;
                align-items: center;
                justify-content: center;
                width: 55px;
                height: 55px;
                border-radius: 100%;
                padding: 10px;
                margin-top: 10px;
                background-color: var(--secondary-background);
                cursor: pointer;
                transition: 0.3s ease;
            }

            .guilds_sidebar > .homeButton img {
                height: 25px;
                object-fit: contain;
            }

            .guilds_sidebar > .homeButton:hover, .guilds_sidebar > .homeButton.active {
                border-radius: 30%;
                background-color: var(--blurple);
                transition: 0.3s ease;
            }
            
            .guilds_sidebar > .actionButton {
                display: grid;
                place-item: center;
                align-items: center;
                justify-content: center;
                width: 55px;
                height: 55px;
                border-radius: 100%;
                padding: 10px;
                margin-bottom: 10px;
                background-color: var(--secondary-background);
                cursor: pointer;
                transition: 0.3s ease;
            }

            .guilds_sidebar > .actionButton svg {
                height: 25px;
                fill: var(--green);
                transition: 0.3s ease;
            }

            .guilds_sidebar > .actionButton:hover{
                border-radius: 30%;
                background-color: var(--green);
                transition: 0.3s ease;
            }

            .guilds_sidebar > .actionButton:hover svg {
                fill: var(--white);
                transition: 0.3s ease;
            }
        "</style>
    }.into_view()
}
