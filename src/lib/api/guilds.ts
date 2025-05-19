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

export async function loadGuild(guildId: string) {
    try {
        const guild: any = await invoke('get_discord_guild', { token: get(TOKEN), guildId });
        GUILDS.update(guilds => {
            const index = guilds.findIndex(g => g.id === guildId);
            if (index !== -1) {
                const permissions = guilds[index].permissions;
                guilds[index] = guild;
                guilds[index].permissions = permissions;
            } else {
                guilds.push(guild);
            }
            return guilds;
        });
        console.log("Guild loaded:", guild);
    } catch (error) {
        console.error("Error loading guild:", error);
    }
}