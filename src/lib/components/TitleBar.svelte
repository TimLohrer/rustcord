<script lang="ts">
    import { ACTIVE_GUILD_ID, GUILDS } from "$lib/stores/stateStore";
    import DiscordLogo from '$lib/assets/logo_white.png';
    import { DiscordAssetUtils } from "$lib/utils/discordAssetUtils";

    $: activeGuild = $GUILDS.find(g => g.id === $ACTIVE_GUILD_ID);
</script>

<div class="title-bar">
    <div class="active-guild">
        {#if $ACTIVE_GUILD_ID === 'HOME'}
            <img src={DiscordLogo} alt="Discord Logo" style="height: 15px; border-radius: 0px;" />
            <p>Direct Messages</p>
        {:else}
            <div class="icon" class:no-icon={!activeGuild?.icon}>
                {#if activeGuild?.icon}
                    <img src={DiscordAssetUtils.getGuildIconUrl($ACTIVE_GUILD_ID, activeGuild?.icon)} alt={activeGuild?.name} />
                {:else}
                    <p>{activeGuild?.name.split('')[0].toUpperCase()}</p>
                {/if}
            </div>
            <p>{activeGuild?.name}</p>
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