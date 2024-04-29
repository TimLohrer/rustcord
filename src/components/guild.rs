use std::cmp::Ordering;

use leptos::*;
use rustcord_lib::data::discord::app_data::AppData;
use rustcord_lib::data::channel::channel::Channel;
use rustcord_lib::data::guild::guild::Guild;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use crate::components::category_channel::CategoryChannel;
use crate::components::channel::Channel as ChannelComponent;
use crate::components::message::Message as MessageComponent;
use crate::AppState;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GetGuildArgs<'a> {
    token: &'a str,
    #[serde(rename = "guildId")]
    guild_id: &'a str,
}

#[derive(Serialize, Deserialize)]
struct GetGuildChannelsArgs<'a> {
    token: &'a str,
    #[serde(rename = "guildId")]
    guild_id: &'a str,
}

#[component]
pub fn Guild(
    state: ReadSignal<AppState>,
    set_state: WriteSignal<AppState>,
    app_data: ReadSignal<AppData>,
    set_app_data: WriteSignal<AppData>,
) -> impl IntoView {
    let mut guild_channels: Vec<Channel> = vec![];
    let fetch_guild = move || {
        spawn_local(async move {
            let mut app_data = app_data.get();

            let get_guild_args = to_value(&GetGuildArgs { token: &app_data.token, guild_id: &state.get().active_guild_id }).unwrap();
            let mut guild: Guild = serde_wasm_bindgen::from_value(invoke("get_discord_guild", get_guild_args).await).unwrap();
            let get_guild_channels_args = to_value(&GetGuildChannelsArgs { token: &app_data.token, guild_id: &guild.id }).unwrap();
            guild.channels = serde_wasm_bindgen::from_value(invoke("get_discord_guild_channels", get_guild_channels_args).await).unwrap();

            // logging::log!("Fetched guild channels: {:?}", &guild.channels);
            
            // sort channels
            guild.channels.iter().for_each(|channels| {
                channels.clone().sort_by(|a, b| {
                    if a.r#type == 4 {
                        Ordering::Less
                    } else if b.r#type == 4 {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
                channels.to_owned().into_iter().filter(|channel| channel.r#type == 4 || channel.parent_id.is_none()).for_each(|mut channel| {
                    if channel.r#type == 4 {
                        let children = channels.clone().into_iter().filter(|child| child.parent_id == Some(channel.id.clone())).collect::<Vec<Channel>>();
                        children.clone().sort_by(|a, b| a.position.cmp(&b.position));
                        logging::log!("Sorted category channels: {:?}", children);

                        channel.children = Some(children.clone());
                    }
                    guild_channels.push(channel.clone());
                })
            });
            
            
            guild.channels = Some(guild_channels.clone());
            app_data.guilds.push(guild.clone());
            set_app_data.set(app_data.clone());
            // logging::log!("Fetched guild: {:?}", &guild);
        });
    };

    if app_data.clone().get().guilds.iter().find(|guild| guild.id == state.get().active_guild_id).is_none() {
        logging::log!("Fetching guild {}...", &state.get().active_guild_id);
        fetch_guild();
    }

    view! {
        <div class={"guild"}>
            {app_data.clone().get().guilds.into_iter().find(|guild| guild.id == state.clone().get().active_guild_id).map(|guild| {
                view! {
                    <div class="channelSidebar">
                        <div class={"guildHeader"}>
                            <div class={"guildName"}>{&guild.name}</div>
                        </div>
                        <div class={"guildChannels"}>
                            {guild.channels.as_ref().unwrap().into_iter().map(|channel| {
                                if channel.r#type == 4 {
                                    view! {
                                        <CategoryChannel state=state set_state=set_state app_data=app_data set_app_data=set_app_data channel=channel.clone() />
                                    }
                                } else {
                                    view! {
                                        <ChannelComponent state=state set_state=set_state app_data=app_data set_app_data=set_app_data channel=channel.clone() />
                                    }
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                    <div class="messages">
                        {app_data.clone().get().message_cache.get(&state.clone().get().active_channel_id).map(|messages| messages.clone().into_iter().map(|message| {
                            view! {
                                <MessageComponent state=state set_state=set_state app_data=app_data set_app_data=set_app_data message=message.clone() />
                            }
                        }).collect::<Vec<_>>().into_view())}
                    </div>
                }.into_view()
            })}
        </div>
        <style>"
            .guild {
                display: flex;
                height: 100vh;
                overflow: hidden;
                padding: 0 5px 0 5px;
                color: var(--secondary-text);
                background-color: var(--secondary-background);
            }

            .guild > .channelSidebar {
                display: flex;
                flex-direction: column;
                width: 250px;
                overflow-y: auto;
                padding: 0 5px 0 5px;
            }

            .guild > .channelSidebar::-webkit-scrollbar {
                width: 5px;
            }

            .guild > .guildHeader {
                display: flex;
                align-items: center;
                height: 50px;
                background-color: var(--discord-dark);
                color: var(--discord-light);
                padding: 0 15px;
                top: 0;
                position: sticky;
                border-shadow: 0 1px 0 rgba(0,0,0,.2);
            }
            
            .guild > .guildChannels {
                display: flex;
                flex-direction: column;
            }
            
            .guild > .messages {
                display: flex;
                flex-direction: column;
                flex: 1;
                overflow-y: auto;
                width: 50vw;
                padding: 0 5px 0 5px;
                background-color: var(--primary-background);
            }

            .guild > .messages::-webkit-scrollbar {
                width: 20px;
            }
        "</style>
    }.into_view()
}
