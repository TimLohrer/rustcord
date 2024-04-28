use leptos::*;
use wasm_bindgen::prelude::*;

use rustcord_lib::data::discord::user::User;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn SidebarUser(
    user: User,
) -> impl IntoView {
    view! {
        // <div class="sidebarUser">
        //     <p class="username">{&user.username} {if user.clone().bot {view! {<p class="userLabel">{"[BOT]"}</p> }} else {view! {<p></p>}}}</p>
        //     <img src={format!("https://cdn.discordapp.com/avatars/{}/{}.webp?size=128", &user.id, &user.avatar.unwrap_or("none".to_string()))} alt="GUILD_ICON" />
        // </div>
    }
}
