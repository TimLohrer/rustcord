import { ACTIVE_USER, DM_CHANNELS, FRIENDS, GATEWAY_SESSION_ID, GUILDS, SETTINGS, TOKEN, VOICE_SOUND_STATE, VOICE_STATE } from "$lib/stores/stateStore";
import { invoke } from "@tauri-apps/api/core";
import { get } from "svelte/store";

export async function connectToGateway(): Promise<void> {
    await invoke("connect_to_gateway", { token: get(TOKEN) });
}

export async function joinVoiceChannel(guildId: string, channelId: string): Promise<void> {
    await invoke("join_voice_channel", {
        guildId,
        channelId,
        mute: get(VOICE_SOUND_STATE) !== 'default',
        deaf: get(VOICE_SOUND_STATE) === 'deaf'
    });
}

export async function leaveVoiceChannel(guildId: string): Promise<void> {
    await invoke("leave_voice_channel", { guildId });
}

export async function handleGatewayEvent(e: any) {
    const eventName: string = e.payload.t;
    const data: any = e.payload.d;

    if (eventName === 'READY') {
        console.log('READY EVENT', data);
        GATEWAY_SESSION_ID.set(data.session_id);
        ACTIVE_USER.set(data.user);
        SETTINGS.set(data.user_settings);
        GUILDS.set(data.guilds);
        DM_CHANNELS.set(data.private_channels);
        FRIENDS.set(data.relationships);
    } else if (eventName === 'VOICE_STATE_UPDATE') {
        if (data.user_id === get(ACTIVE_USER).id && data.session_id === get(GATEWAY_SESSION_ID)) {
            console.log('STATE', get(VOICE_STATE));
            console.log('DATA', data);
            if (data.channel_id === null) {
                VOICE_STATE.set(null);
            } else if (data.guild_id !== null && (data.cannel_id !== get(VOICE_STATE)?.channelId || data.guild_id !== get(VOICE_STATE)?.guildId)) {
                VOICE_STATE.set({
                    guildId: data.guild_id,
                    channelId: data.channel_id,
                    startTime: new Date().getTime()
                });
            }

            if (data.self_mute && !data.self_deaf && get(VOICE_SOUND_STATE) !== 'mute'
                || data.self_mute && data.self_deaf && get(VOICE_SOUND_STATE) !== 'deaf'
                || !data.self_mute && !data.self_deaf && get(VOICE_SOUND_STATE) !== 'default') {
                VOICE_SOUND_STATE.set(data.self_mute ? (data.self_deaf ? 'deaf' : 'mute') : 'default');
            }
        } else if (data.user_id === get(ACTIVE_USER).id && data.guild_id === get(VOICE_STATE)?.guildId && data.session_id !== get(GATEWAY_SESSION_ID)) {
            // Connected to the same guild but different session -> Display notification in the future...
            VOICE_STATE.set(null);
        }
    }
}