<script lang="ts">
    import { ACTIVE_USER, VOICE_STATE } from "$lib/stores/stateStore";
    import { hexToRGBAString } from "$lib/utils/colorUtils";
    import { DiscordIcons } from "$lib/utils/iconUtils";
    import { onMount, tick } from "svelte";

    $: muteIcon = $VOICE_STATE == 'default'
        ? muteHovered
            ? DiscordIcons.withColor(DiscordIcons.MUTE, 'white')
            : DiscordIcons.withColor(DiscordIcons.MUTE, hexToRGBAString('var(--secondary-text)'))
        : DiscordIcons.withColor(DiscordIcons.MUTED, hexToRGBAString('var(--red)'));

    $: deafIcon = $VOICE_STATE !== 'deaf'
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
        
    function muteHover() {
        muteHovered = true;
        const muteButton = document.querySelector('.mute-button') as HTMLDivElement;
        if (muteButton) {
            muteButton.style.backgroundColor = $VOICE_STATE !== 'default' ? hexToRGBAString('var(--red)', 0.2) : hexToRGBAString('var(--white)', 0.1);
        }
    }

    function muteHoverEnd() {
        muteHovered = false;
        const muteButton = document.querySelector('.mute-button') as HTMLDivElement;
        if (muteButton) {
            muteButton.style.backgroundColor = $VOICE_STATE !== 'default' ? hexToRGBAString('var(--red)', 0.1) : 'transparent';
        }
    }

    function deafHover() {
        deafHovered = true;
        const deafButton = document.querySelector('.deaf-button') as HTMLDivElement;
        if (deafButton) {
            deafButton.style.backgroundColor = $VOICE_STATE === 'deaf' ? hexToRGBAString('var(--red)', 0.2) : hexToRGBAString('var(--white)', 0.1);
        }
    }

    function deafHoverEnd() {
        deafHovered = false;
        const deafButton = document.querySelector('.deaf-button') as HTMLDivElement;
        if (deafButton) {
            deafButton.style.backgroundColor = $VOICE_STATE === 'deaf' ? hexToRGBAString('var(--red)', 0.1) : 'transparent';
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
        if ($VOICE_STATE === 'default') {
            VOICE_STATE.set('mute');
        } else {
            VOICE_STATE.set('default');
        }
        muteHoverEnd();
    }

    function deaf() {
        if ($VOICE_STATE !== 'deaf') {
            VOICE_STATE.set('deaf');
        } else {
            VOICE_STATE.set('default');
        }
        muteHoverEnd();
        deafHoverEnd();
    }

    function openSettings() {
        alert('Settings clicked');
        settingsHoverEnd();
    }

    onMount(() => {
        muteHoverEnd();
        deafHoverEnd();
    })
</script>

<div class="active-user-card">
    <div class="user">
        <img src={`https://cdn.discordapp.com/avatars/${$ACTIVE_USER.id}/${$ACTIVE_USER.avatar}.webp?size=80`} alt="Avatar" class="avatar">
        <div class="user-info">
            <p class="name">{$ACTIVE_USER.global_name}</p>
            <div class="status-username-wrapper">
                <p class="status">STatus LOOLOLOLol</p>
                <p class="username">{$ACTIVE_USER.username}</p>
            </div>
        </div>
    </div>
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div class="buttons">
        <div class="button mute-button" on:click={mute} on:mouseenter={muteHover} on:mouseleave={muteHoverEnd}>
            {@html muteIcon}
        </div>
        <div class="button deaf-button" on:click={deaf} on:mouseenter={deafHover} on:mouseleave={deafHoverEnd}>
            {@html deafIcon}
        </div>
        <div class="button settings-button" on:click={openSettings} on:mouseenter={settingsHover} on:mouseleave={settingsHoverEnd}>
            {@html settingsIcon}
        </div>
    </div>
</div>

<style>
    .active-user-card {
        display: flex;
        flex-direction: row;
        width: 100%;
        height: 100%;
        background-color: var(--primary-background);
		border-radius: inherit;
        padding: 10px;
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

    .active-user-card:hover .status, .active-user-card:hover .username {
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

    .active-user-card .user .user-info .status-username-wrapper .status {
        font-size: 0.8rem;
        color: var(--secondary-text);
        font-weight: 600;
        transition-duration: 200ms;
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
</style>