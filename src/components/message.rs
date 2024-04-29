use leptos::{leptos_dom::logging, *};
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

    let create_timestamp: String = {
        let timestamp = message.clone().timestamp.replace("T", " ");
        let parts: Vec<String> = timestamp.split('.').map(|s| s.to_string()).collect();
        parts.get(0).unwrap_or(&String::from("")).to_string()
    };

    view! {
        <div class="message">
            <div class="authorAvatarContainer">
                <img class="authorAvatar" src={if message.clone().author.avatar.is_some() { format!("https://cdn.discordapp.com/avatars/{}/{}.webp?size=128", message.clone().author.id, message.clone().author.avatar.unwrap()) } else { String::from("../../public/assets/discord/default_pfp.png") }} alt="user pfp" />
            </div>
            <div class="body">
                <div class="header">
                    <p class="author">{if message.clone().author.global_name.is_some() { message.clone().author.global_name.unwrap() } else { message.clone().author.username }}</p>
                    {if message.clone().author.bot.unwrap_or_default() || message.clone().author.system.unwrap_or_default() || message.clone().webhook_id.is_some() {
                        view! {
                            <div class="appLabel">
                                <p>{if message.clone().author.system.unwrap_or_default() { "SYSTEM" } else if message.clone().webhook_id.is_some() { "WEBHOOK" } else if message.author.bot.unwrap_or_default() && message.author.verified.unwrap_or_default() { "✔️ APP" } else { "APP" }}</p>
                            </div>
                        }.into_view()
                    } else {
                        view! {}.into_view()
                    }}
                    <p class="timestamp">{&create_timestamp}</p>
                </div>
                <p class="content">{&message.content}</p>
                {if message.clone().sticker_items.is_some() {
                    {message.clone().sticker_items.unwrap().iter().map(|sticker| {
                        view! {
                            <p>Sticker {&sticker.name}</p>
                        }
                    }).collect::<Vec<_>>().into_view()}  
                } else {
                    view! {}.into_view()
                }}
                {if message.clone().attachments.iter().filter(|attachment| attachment.content_type.clone().unwrap_or_default().contains("audio")).count() > 0 {
                    {message.clone().attachments.iter().filter(|attachment| attachment.content_type.clone().unwrap_or_default().contains("audio")).map(|attachment| {
                        view! {
                            <audio controls>
                                <source src={attachment.url.clone().unwrap_or_default()} type={attachment.content_type.clone()} />
                            </audio>
                            // FIXME: Audio not working??
                        }
                    }).collect::<Vec<_>>().into_view()}
                } else {
                    view! {}.into_view()
                }}
                {if message.clone().attachments.iter().filter(|attachment| attachment.content_type.clone().unwrap_or_default().contains("image")).count() > 0 {
                    {message.clone().attachments.iter().filter(|attachment| attachment.content_type.clone().unwrap_or_default().contains("image")).map(|attachment| {
                        view! {
                            <img class="attachment" src={attachment.url.clone().unwrap_or_default()} alt="attachment" />
                        }
                    }).collect::<Vec<_>>().into_view()}
                } else {
                    view! {}.into_view()
                }}
                {if message.clone().embeds.len() > 0 {
                    {message.clone().embeds.iter().filter(|embed| embed.r#type.to_lowercase() == "rich").map(|embed| {
                        logging::console_log(&format!("{:?}", embed));
                        view! {
                            <div class="embed">
                                <div class="sideColor" style={ format!("{}; width: 3.5px; border-top-left-radius: 5px; border-bottom-left-radius: 5px; height: 100%;", if embed.color.clone().is_some() { format!("background-color: #{:x};", embed.color.clone().unwrap()) } else { String::from("background-color: var(--discord-dark);") }) }></div>
                                <div class="embedContent">
                                    {if embed.author.clone().is_some() {
                                        view! {
                                            <div class="embedHeader">
                                                {if embed.author.clone().unwrap().icon_url.is_some() {
                                                    view! {
                                                        <img class="embedAuthorAvatar" src={embed.author.clone().unwrap().icon_url.unwrap_or_default()} alt="embed author pfp" />
                                                    }.into_view()
                                                } else {
                                                    view! {}.into_view()
                                                }}
                                                <p class="embedAuthor">{embed.author.clone().unwrap().name.clone()}</p>
                                            </div>
                                        }.into_view()
                                    } else {
                                        view! {}.into_view()
                                    }}
                                    <p class="embedTitle">{embed.title.clone().unwrap_or_default()}</p>
                                    <p class="embedDescription">{embed.description.clone().unwrap_or_default()}</p>
                                    <div class="fields">
                                        {if embed.clone().fields.is_some() {
                                            view! {
                                                {embed.fields.clone().unwrap().iter().map(|field| {
                                                    view! {
                                                        <div class="field">
                                                            <p class="fieldName">{field.name.clone()}</p>
                                                            <p class="fieldValue">{field.value.clone()}</p>
                                                        </div>
                                                    }
                                                }).collect::<Vec<_>>().into_view()}
                                            }
                                        } else {
                                            view! {}.into_view()
                                        }}
                                    </div>
                                    {if embed.footer.clone().is_some() {
                                        view! {
                                            <p class="embedFooter">{embed.footer.clone().unwrap().text.clone()}</p>
                                        }.into_view()
                                    } else {
                                        view! {}.into_view()
                                    }}
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>().into_view()}
                } else {
                    view! {}.into_view()
                }}
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
                background-color: var(--secondary-background-hover);
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
                font-weight: 600;
            }

            .message > .body > .header > .appLabel {
                display: flex;
                justify-content: center;
                align-items: center;
                height: 17px;
                padding: 1.5px 5px 1.5px 5px;
                margin-left: 5px;
                border-radius: 5px;
                background-color: var(--blurple);
            }
            
            .message > .body > .header > .appLabel > p {
                font-size: 11px;
                font-weight: 600;
                padding-top: 3.5px;
            }
        
            .message > .body > .header > .timestamp {
                font-size: 13.5px;
                font-weight: 400;
                margin-left: 10px;
                color: var(--secondary-text);
            }
        
            .message > .body > .content {
                font-size: 15px;
                font-weight: 400;
            }

            .message > .body > .attachment {
                max-height: 350px;
                border-radius: 7.5px;
                margin-top: 5px;
            }

            .message > .body > .embed {
                display: grid;
                grid-auto-flow: column;
                grid-auto-rows: 100%;
                border-radius: 5px;
                margin-top: 15px;
                max-width: 500px;
                background-color: var(--secondary-background);
            }

            .message > .body > .embed > .embedContent {
                display: flex;
                flex-direction: column;
                justify-content: flex-start;
                align-items: flex-start;
                padding: 10px;
            }

            .message > .body > .embed > .embedContent > .embedHeader {
                display: flex;
                flex-direction: row;
                justify-content: flex-start;
                align-items: center;
                margin-bottom: 5px;
            }

            .message > .body > .embed > .embedContent > .embedHeader > .embedAuthorAvatar {
                width: 25px;
                height: 25px;
                border-radius: 50%;
                margin-right: 10px;
            }

            .message > .body > .embed > .embedContent > .embedHeader > .embedAuthor {
                font-size: 15px;
                font-weight: 600;
            }

            .message > .body > .embed > .embedContent > .embedTitle {
                font-size: 22.5px;
                font-weight: 600;
                margin-bottom: 5px;
            }

            .message > .body > .embed > .embedContent > .embedDescription {
                font-size: 15px;
                font-weight: 400;
                margin-bottom: 5px;
            }

            .message > .body > .embed > .embedContent > .fields {
                display: flex;
                flex-direction: row;
                justify-content: flex-start;
                align-items: flex-start;
                margin-bottom: 5px;
            }

            .message > .body > .embed > .embedContent > .fields > .field {
                display: flex;
                flex-direction: column;
                justify-content: flex-start;
                align-items: flex-start;
                margin-right: 10px;
            }

            .message > .body > .embed > .embedContent > .fields > .field > .fieldName {
                font-size: 15px;
                font-weight: 600;
            }

            .message > .body > .embed > .embedContent > .fields > .field > .fieldValue {
                font-size: 15px;
                font-weight: 400;
            }

            .message > .body > .embed > .embedContent > .embedFooter {
                font-size: 13px;
                font-weight: 400;
                margin-top: 5px;
            }
        "</style>
    }
}