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
pub fn CurrentUserPanel(
    state: ReadSignal<AppState>,
    set_app_state: WriteSignal<AppState>,
    app_data: ReadSignal<AppData>,
) -> impl IntoView {
    view! {
        <div class={"current_user_panel"}>
            
        </div>

        <style>"
        "</style>
    }.into_view()
}
