import { writable, type Writable } from "svelte/store";

export const TOKEN = writable("");
export const SETTINGS: Writable<null | any> = writable(null);
export const ACTIVE_USER: Writable<null | any> = writable(null);
export const ACTIVE_GUILD_ID = writable("HOME");

export const VOICE_STATE: Writable<'default' | 'mute' | 'deaf'> = writable('default');

export const DM_CHANNELS: Writable<any[]> = writable([]);

export const GUILDS: Writable<any[]> = writable([]);
export const FRIENDS = writable([]);