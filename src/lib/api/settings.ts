import { SETTINGS, TOKEN } from "$lib/stores/stateStore";
import { invoke } from "@tauri-apps/api/core";
import { get } from "svelte/store";

export async function loadSettings(): Promise<void> {
    try {
        const settings = await invoke("get_discord_settings", { token: get(TOKEN) });
        SETTINGS.set(settings);
        console.log("Settings loaded:", settings);
    } catch (error) {
        console.error("Error loading settings:", error);
    }
}