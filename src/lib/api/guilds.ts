import { GUILDS, TOKEN } from "$lib/stores/stateStore";
import { invoke } from "@tauri-apps/api/core";
import { get } from "svelte/store";

export async function loadGuilds() {
    try {
        const guilds: any[] = await invoke('get_discord_guilds', { token: get(TOKEN) });
        GUILDS.set(guilds);
        console.log("Guilds loaded:", guilds);
    } catch (error) {
        console.error("Error loading guilds:", error);
        GUILDS.set([]);
    }
}