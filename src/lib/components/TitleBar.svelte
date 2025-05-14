<script lang="ts">
    import { ACTIVE_GUILD_ID, GUILDS } from "$lib/stores/stateStore";
    import DiscordLogo from '$lib/assets/logo_white.png';
    import { DiscordAssetUtils } from "$lib/utils/discordAssetUtils";

    $: activeGuild = $GUILDS.find(g => g.id === $ACTIVE_GUILD_ID);
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="title-bar" data-tauri-drag-region>
    <div class="active-guild" data-tauri-drag-region>
        {#if $ACTIVE_GUILD_ID === 'HOME'}
            <img src={DiscordLogo} alt="Discord Logo" style="height: 15px; border-radius: 0px;" data-tauri-drag-region />
            <p data-tauri-drag-region>Direct Messages</p>
        {:else}
            <div class="icon" class:no-icon={!activeGuild?.icon} data-tauri-drag-region>
                {#if activeGuild?.icon}
                    <img src={DiscordAssetUtils.getGuildIconUrl($ACTIVE_GUILD_ID, activeGuild?.icon)} alt={activeGuild?.name} data-tauri-drag-region />
                {:else}
                    <p data-tauri-drag-region>{activeGuild?.name.split('')[0].toUpperCase()}</p>
                {/if}
            </div>
            <p data-tauri-drag-region>{activeGuild?.name}</p>
        {/if}
    </div>
</div>

<style>
    .title-bar {
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
    }

    .active-guild {
        display: flex;
        flex-direction: row;
        align-items: center;
        pointer-events: none;
        gap: 10px;
    }

    .active-guild .icon {
        width: 20px;
        height: 20px;
        border-radius: 5px;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .active-guild .icon.no-icon {
        background-color: var(--primary-background);
    }

    .active-guild .icon img {
        height: 100%;
        width: 100%;
        object-fit: cover;
        border-radius: 5px;
    }

    .active-guild .icon p {
        font-size: 14px;
        color: var(--white);
        font-weight: 500;
    }

    .active-guild p {
        font-size: 16px;
        color: var(--white);
        font-weight: 500;
    }
</style>