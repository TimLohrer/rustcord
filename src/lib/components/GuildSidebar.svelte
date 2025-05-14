<script lang="ts">
    import DiscordLogo from '$lib/assets/logo_white.png';
    import { ACTIVE_GUILD_ID, GUILDS } from '$lib/stores/stateStore';
    import { DiscordAssetUtils } from '$lib/utils/discordAssetUtils';
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div class="guilds-sidebar">
    <div class="guild-wrapper" class:active={$ACTIVE_GUILD_ID === 'HOME'}>
        <div class="pill-slot">
            <!-- svelte-ignore element_invalid_self_closing_tag -->
            <div class="pill" class:active={$ACTIVE_GUILD_ID === 'HOME'} />
        </div>
        <div class="home-button guild" class:active={$ACTIVE_GUILD_ID === 'HOME'} on:click={() => ACTIVE_GUILD_ID.set('HOME')}>
            <img src={DiscordLogo} alt="Discord Logo" />
        </div>
    </div>
    <!-- svelte-ignore element_invalid_self_closing_tag -->
    <div class="divider" />
    {#each $GUILDS as guild}
        <div class="guild-wrapper" class:active={$ACTIVE_GUILD_ID === guild.id}>
            <div class="pill-slot">
                <!-- svelte-ignore element_invalid_self_closing_tag -->
                <div class="pill" class:active={$ACTIVE_GUILD_ID === guild.id} />
            </div>
            <div class="guild" on:click={() => ACTIVE_GUILD_ID.set(guild.id)}>
                <img src={DiscordAssetUtils.getGuildIconUrl(guild.id, guild.icon)} alt={guild.name} />
            </div>
        </div>
    {/each}
    <!-- <div class="guild-wrapper" class:active={$ACTIVE_GUILD_ID === 'HOME'}>
        <div class="home-button guild" class:active={$ACTIVE_GUILD_ID === 'HOME'} on:click={() => ACTIVE_GUILD_ID.set('HOME')}>
            <img src={DiscordLogo} alt="Discord Logo" />
        </div>
    </div>
    <div class="guild-wrapper" class:active={$ACTIVE_GUILD_ID === 'HOME'}>
        <div class="home-button guild" class:active={$ACTIVE_GUILD_ID === 'HOME'} on:click={() => ACTIVE_GUILD_ID.set('HOME')}>
            <img src={DiscordLogo} alt="Discord Logo" />
        </div>
    </div> -->
</div>

<style>
    .guilds-sidebar {
        width: 100%;
        height: 100%;
        display: flex;
        align-items: start;
        flex-direction: column;
        gap: 7.5px;
        overflow-y: scroll;
    }

    .guilds-sidebar::-webkit-scrollbar {
        width: 0px;
        background: transparent;
    }

    .guilds-sidebar .guild-wrapper {
        width: calc(100% - 17.5px);
        display: flex;
        justify-content: space-between;
        align-items: center;
        transition-duration: 200ms;
    }

    .guilds-sidebar .home-button {
        background-color: var(--primary-background);
    }

    .guilds-sidebar .home-button img {
        height: 20px;
        margin: 15px 0 15px 0;
    }

    .guilds-sidebar .home-button:hover, .guilds-sidebar .home-button.active {
        background-color: var(--blurple);
    }

    .guilds-sidebar .divider {
        display: flex;
        width: 45%;
        min-height: 2px;
        align-self: center;
        background-color: var(--border-color);
    }

    .guilds-sidebar .guild {
        width: 50px;
        height: 50px;
        border-radius: 15px;
        display: flex;
        justify-content: center;
        align-items: center;
        cursor: pointer;
        transition-duration: 200ms;
    }

    .guilds-sidebar .guild:not(.home-button) img {
        width: 100%;
        height: 100%;
        object-fit: cover;
        border-radius: 15px;
    }

    .guilds-sidebar .pill-slot {
        width: 5px;
        display: flex;
        min-height: 50px;
        flex-direction: column;
        justify-content: center;
    }

    .guilds-sidebar .pill-slot .pill {
        height: 0px;
        width: 5px;
        border-top-right-radius: 10px;
        border-bottom-right-radius: 10px;
        background-color: white;
        transition-duration: 100ms;
    }

    .guilds-sidebar .guild-wrapper:not(.active):hover .pill-slot .pill {
        height: 25px;
    }

    .guilds-sidebar .pill-slot .pill.active {
        height: 45px;
    }
</style>