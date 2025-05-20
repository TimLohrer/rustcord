<script lang="ts">
    import { ACTIVE_USER, GUILDS, SETTINGS, VOICE_SOUND_STATE, VOICE_STATE } from "$lib/stores/stateStore";
    import { hexToRGBAString } from "$lib/utils/colorUtils";
    import { DiscordAssetUtils } from "$lib/utils/discordAssetUtils";
    import { DiscordIcons } from "$lib/utils/iconUtils";
    import { onMount, tick } from "svelte";

    $: muteIcon = $VOICE_SOUND_STATE == 'default'
        ? muteHovered
            ? DiscordIcons.withColor(DiscordIcons.MUTE, 'white')
            : DiscordIcons.withColor(DiscordIcons.MUTE, hexToRGBAString('var(--secondary-text)'))
        : DiscordIcons.withColor(DiscordIcons.MUTED, hexToRGBAString('var(--red)'));

    $: deafIcon = $VOICE_SOUND_STATE !== 'deaf'
        ? deafHovered
            ? DiscordIcons.withColor(DiscordIcons.DEAFEN, 'white')
            : DiscordIcons.withColor(DiscordIcons.DEAFEN, hexToRGBAString('var(--secondary-text)'))
        : DiscordIcons.withColor(DiscordIcons.DEAFENED, hexToRGBAString('var(--red)'));

    $: settingsIcon =
        settingsHovered
            ? DiscordIcons.withColor(DiscordIcons.SETTINGS, 'white')
            : DiscordIcons.withColor(DiscordIcons.SETTINGS, hexToRGBAString('var(--secondary-text)'));

    let muteHovered = false;
    let deafHovered = false;
    let settingsHovered = false;

    let callTime = '';
        
    function muteHover() {
        muteHovered = true;
        const muteButton = document.querySelector('.mute-button') as HTMLDivElement;
        if (muteButton) {
            muteButton.style.backgroundColor = $VOICE_SOUND_STATE !== 'default' ? hexToRGBAString('var(--red)', 0.2) : hexToRGBAString('var(--white)', 0.1);
        }
    }

    function muteHoverEnd() {
        muteHovered = false;
        const muteButton = document.querySelector('.mute-button') as HTMLDivElement;
        if (muteButton) {
            muteButton.style.backgroundColor = $VOICE_SOUND_STATE !== 'default' ? hexToRGBAString('var(--red)', 0.1) : 'transparent';
        }
    }

    function deafHover() {
        deafHovered = true;
        const deafButton = document.querySelector('.deaf-button') as HTMLDivElement;
        if (deafButton) {
            deafButton.style.backgroundColor = $VOICE_SOUND_STATE === 'deaf' ? hexToRGBAString('var(--red)', 0.2) : hexToRGBAString('var(--white)', 0.1);
        }
    }

    function deafHoverEnd() {
        deafHovered = false;
        const deafButton = document.querySelector('.deaf-button') as HTMLDivElement;
        if (deafButton) {
            deafButton.style.backgroundColor = $VOICE_SOUND_STATE === 'deaf' ? hexToRGBAString('var(--red)', 0.1) : 'transparent';
        }
    }

    async function settingsHover() {
        settingsHovered = true;
        // Tick here to ensure the DOM is updated before running the animation
        await tick();
        const settingsButton = document.querySelector('.settings-button') as HTMLDivElement;
        const settingsButtonSvg = document.querySelector('.settings-icon-svg') as SVGElement;
        if (settingsButton && settingsButtonSvg) {
            settingsButton.style.backgroundColor = hexToRGBAString('var(--white)', 0.1);
            settingsButtonSvg.animate([
                { transform: 'rotate(0deg)' },
                { transform: 'rotate(360deg)' }
            ], {
                duration: 1250,
                easing: 'ease-in-out'
            });
        }
    }

    function settingsHoverEnd() {
        settingsHovered = false;
        const settingsButton = document.querySelector('.settings-button') as HTMLDivElement;
        if (settingsButton) {
            settingsButton.style.backgroundColor = 'transparent';
        }
    }

    function mute() {
        if ($VOICE_SOUND_STATE === 'default') {
            VOICE_SOUND_STATE.set('mute');
        } else {
            VOICE_SOUND_STATE.set('default');
        }
        muteHoverEnd();
    }

    function deaf() {
        if ($VOICE_SOUND_STATE !== 'deaf') {
            VOICE_SOUND_STATE.set('deaf');
        } else {
            VOICE_SOUND_STATE.set('default');
        }
        muteHoverEnd();
        deafHoverEnd();
    }

    function openSettings() {
        alert('Settings clicked');
        settingsHoverEnd();
    }

    function getCallTime() {
        if ($VOICE_STATE !== null) {
            const startTime = $VOICE_STATE.startTime;
            const currentTime = new Date().getTime();
            const diff = Math.floor((currentTime - startTime) / 1000);
            const padLeft = (num: number) => (num < 10 ? '0' : '') + num;
            switch (true) {
                case diff < 3600:
                    callTime = `${padLeft(Math.floor(diff / 60))}:${padLeft(diff % 60)}`;
                    break;
                case diff < 86400:
                    callTime = `${padLeft(Math.floor(diff / 3600))}:${padLeft(Math.floor((diff % 3600) / 60))}:${padLeft(diff % 60)}`;
                    break;
                case diff < 604800:
                    callTime = `${padLeft(Math.floor(diff / 86400))}:${padLeft(Math.floor((diff % 86400) / 3600))}:${padLeft(Math.floor((diff % 3600) / 60))}`;
                    break;
            }
        }
    }

    onMount(() => {
        muteHoverEnd();
        deafHoverEnd();

        getCallTime();
        const interval = setInterval(() => {
            getCallTime();
        }, 1000);

        return () => {
            clearInterval(interval);
        };
    });
</script>

<div class="active-user-card">
    {#if $VOICE_STATE !== null}
        <div class="voice-call">
            <div class="call-state-wrapper">
                <div class="call-state">
                    <p>Voice Connected</p>
                    <p>{$GUILDS.find((g: any) => g.id === $VOICE_STATE.guildId)?.channels?.find((c: any) => c.id === $VOICE_STATE.channelId).name ?? 'Unknown Channel'} / {$GUILDS.find((g: any) => g.id === $VOICE_STATE.guildId)?.name ?? 'Unknown Guild'}</p>
                    <p>Connected for {callTime}</p>
                </div>
                <div class="call-state-buttons">

                </div>
            </div>
            <div class="media-buttons">
                <div class="button">
                    <p>Camera</p>
                </div>
                <div class="button">
                    <p>Stream</p>
                </div>
            </div>
        </div>
    {/if}
    <div class="user-wrapper">
        <div class="user">
            <img src={DiscordAssetUtils.getUserAvatarUrl($ACTIVE_USER.id, $ACTIVE_USER.avatar)} alt="Avatar" class="avatar">
            <div class="user-info">
                <p class="name">{$ACTIVE_USER.global_name}</p>
                <div class="status-username-wrapper">
                    <div class="status-wrapper">
                        {#if $SETTINGS?.custom_status.emoji_id}
                            <img src={DiscordAssetUtils.getEmojiUrl($SETTINGS?.custom_status.emoji_id, 64, true)} alt={`Emoji: ${$SETTINGS?.custom_status.emoji_name}`} class="status-icon">
                        {/if}
                        <p class="status">{$SETTINGS?.custom_status.text}</p>
                    </div>
                    <p class="username">{$ACTIVE_USER.username}</p>
                </div>
            </div>
        </div>
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_mouse_events_have_key_events -->
        <div class="buttons">
            <div class="button mute-button" on:click={mute} on:mouseover={muteHover} on:mouseleave={muteHoverEnd}>
                {@html muteIcon}
            </div>
            <div class="button deaf-button" on:click={deaf} on:mouseover={deafHover} on:mouseleave={deafHoverEnd}>
                {@html deafIcon}
            </div>
            <div class="button settings-button" on:click={openSettings} on:mouseover={settingsHover} on:mouseleave={settingsHoverEnd}>
                {@html settingsIcon}
            </div>
        </div>
    </div>
</div>

<style>
    .active-user-card {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
        align-items: center;
        background-color: var(--primary-background);
		border-radius: inherit;
        padding: 10px;
    }

    .active-user-card .user-wrapper {
        display: flex;
        flex-direction: row;
        height: 100%;
        align-items: center;
        justify-content: space-between;
    }

    .active-user-card .user {
        display: flex;
        flex-direction: row;
        align-items: center;
        width: 100%;
        margin-top: -2.5px;
        border-radius: 7.5px;
    }
    
    .active-user-card .user:hover {
        background-color: var(--border-color);
    }

    .active-user-card:hover .status-wrapper, .active-user-card:hover .username {
        transform: translateY(-25px);
    }
    
    .active-user-card .user .avatar {
        width: 50px;
        height: 50px;
        border-radius: 50%;
        margin-right: 5px;
    }

    .active-user-card .user .user-info {
        display: flex;
        flex-direction: column;
        justify-content: center;
        height: 50px;
    }

    .active-user-card .user .user-info .name {
        font-size: 1rem;
        font-weight: 600;
        height: 20px;
    }

    .active-user-card .user .user-info .status-username-wrapper {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        height: 20px;
        margin-top: -2.5px;
        overflow: hidden;
    }

    .active-user-card .user .user-info .status-username-wrapper .status-wrapper {
        display: flex;
        flex-direction: row;
        align-items: center;
        white-space: nowrap;
        width: 140px;
        transition-duration: 200ms;
    }
    
    .active-user-card .user .user-info .status-username-wrapper .status-wrapper .status {
        font-size: 0.8rem;
        color: var(--secondary-text);
        font-weight: 600;
    }

    .active-user-card .user .user-info .status-username-wrapper .status-icon {
        width: 15px;
        height: 15px;
        margin-right: 5px;
        border-radius: 50%;
    }
    
    .active-user-card .user .user-info .status-username-wrapper .username {
        font-size: 0.8rem;
        color: var(--secondary-text);
        font-weight: 600;
        transition-duration: 200ms;
    }

    .buttons {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
        height: 100%;
        gap: 10px;
        margin: 0 10px;
    }

    .buttons .button {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 35px;
        height: 35px;
        padding: 5px;
        cursor: pointer;
        border-radius: 7.5px;
        transition-duration: 200ms;
    }

    .active-user-card .voice-call {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
        gap: 10px;
    }

    .active-user-card .voice-call .call-state-wrapper {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: space-between;
        width: 100%;
    }

    .active-user-card .voice-call .call-state-wrapper .call-state {
        display: flex;
        flex-direction: column;
        align-items: start;
        justify-content: center;
    }

    .active-user-card .voice-call .call-state-wrapper .call-state p {
        color: var(--white);
    }

    .active-user-card .voice-call .call-state-wrapper .call-state p:nth-child(1) {
        font-size: 18px;
        font-weight: 500;
        color: var(--green);
    }

    .active-user-card .voice-call .call-state-wrapper .call-state p:nth-child(2) {
        font-size: 14px;
        font-weight: 600;
    }

    .active-user-card .voice-call .call-state-wrapper .call-state p:nth-child(3) {
        font-size: 12px;
        font-weight: 500;
        margin-top: -5px;
    }

    .active-user-card .voice-call .call-state-wrapper .call-state-buttons {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 10px;
    }

    .active-user-card .voice-call .media-buttons {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
        gap: 10px;
        width: 100%;
    }

    .active-user-card .voice-call .media-buttons .button {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 100%;
        height: 50px;
        padding: 5px;
        cursor: pointer;
        border-radius: 7.5px;
        background-color: var(--primary-background);
        border: 1px solid var(--border-color);
        transition-duration: 200ms;
    }

    .active-user-card .voice-call .media-buttons .button:hover {
        background-color: rgba(255, 255, 255, 0.1);
    }
</style>