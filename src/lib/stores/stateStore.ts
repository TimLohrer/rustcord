import { loadGuildChannels } from "$lib/api/channels";
import { loadGuild } from "$lib/api/guilds";
import { get, writable, type Writable } from "svelte/store";

export const TOKEN = writable("");
export const SETTINGS: Writable<null | any> = writable(null);
export const ACTIVE_USER: Writable<null | any> = writable(null);
export const ACTIVE_GUILD_ID = writable("HOME");
export const ACTIVE_CHANNEL_ID: Writable<Record<string, string>> = writable({ "HOME": "HOME" });

export const VOICE_STATE: Writable<'default' | 'mute' | 'deaf'> = writable('default');

export const DM_CHANNELS: Writable<any[]> = writable([]);

export const GUILDS: Writable<any[]> = writable([]);
export const FRIENDS = writable([]);

/// ----- State Logic ----- ///

ACTIVE_GUILD_ID.subscribe(async (guildId) => {
    if (guildId !== 'HOME' && !get(GUILDS).find(g => g.id === guildId)?.channels) {
        console.log("Detected new guild, loading full guild and channels...");
        await loadGuild(guildId);
        await loadGuildChannels(guildId);
    }
});

export function setActiveChannelId(guidId: string, channelId: string) {
    ACTIVE_CHANNEL_ID.update((prev) => {
        return { ...prev, [guidId]: channelId };
    });
}