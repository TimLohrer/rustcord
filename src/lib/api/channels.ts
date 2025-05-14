import { DM_CHANNELS, TOKEN } from "$lib/stores/stateStore";
import { SnowflakeUtils } from "$lib/utils/snowflakeUtils";
import { invoke } from "@tauri-apps/api/core";
import { get } from "svelte/store";

export async function loadDMChannels(): Promise<void> {
    try {
        let channels: any[] = await invoke("get_discord_dm_channels", { token: get(TOKEN) });

        channels.sort((a: any, b: any) => {
            // Sort by last_message_id or channel id if last_message_id is not present
            const lastMessageDateA = SnowflakeUtils.snowflakeToDate(a.last_message_id ?? a.id);
            const lastMessageDateB = SnowflakeUtils.snowflakeToDate(b.last_message_id ?? a.id);

            return lastMessageDateB.getTime() - lastMessageDateA.getTime();
        });
        DM_CHANNELS.set(channels);
        console.log("DM Channels loaded:", channels);
    } catch (error) {
        console.error("Error loading DM channels:", error);
    }
}