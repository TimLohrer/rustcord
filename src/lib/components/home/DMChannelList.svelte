<script lang="ts">
  import { ACTIVE_CHANNEL_ID, DM_CHANNELS, setActiveChannelId } from "$lib/stores/stateStore";
  import { DiscordAssetUtils } from "$lib/utils/discordAssetUtils";
  import { DiscordIcons } from "$lib/utils/iconUtils";
  import DefaultDMGroupAvatar from "$lib/assets/default_dm_group_avatar.png";

  let hoveredChannelCloseButtonId: string | null = null;
  let hoveredChannelId: string | null = null;

  $: activeChannelId = $ACTIVE_CHANNEL_ID['HOME'] ?? 'HOME';
</script>

<div class="dm-channel-list">
    <div class="find-a-conversation-wrapper">
        <div class="find-a-conversation-button">
            <p>Find or start a conversation</p>
        </div>
    </div>
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div class="pinned-buttons">
        <div class="pinned-button" on:click={() => setActiveChannelId('HOME', 'HOME')} class:active={activeChannelId === 'HOME'}>
            <div class="icon-wrapper">
                {@html DiscordIcons.withColor(DiscordIcons.HOME, activeChannelId === 'HOME' ? 'var(--white)' : 'var(--secondary-text)')}
            </div>
            <p>Home</p>
        </div>
        <div class="pinned-button" on:click={() => setActiveChannelId('HOME', 'FRIENDS')} class:active={activeChannelId === 'FRIENDS'}>
            <div class="icon-wrapper">
                {@html DiscordIcons.withColor(DiscordIcons.FRIENDS, activeChannelId === 'FRIENDS' ? 'var(--white)' : 'var(--secondary-text)')}
            </div>
            <p>Friends</p>
        </div>
    </div>
    <!-- svelte-ignore element_invalid_self_closing_tag -->
    <div class="divider" />
    <div class="create-button-wrapper">
        <p>Direct Messages</p>
        <div class="add-icon-wrapper">
            {@html DiscordIcons.withColor(DiscordIcons.ADD, 'var(--secondary-text)')}
        </div>
    </div>
    <div class="dm-channels">
        {#each $DM_CHANNELS ?? [] as channel}
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_mouse_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="dm-channel" class:active={activeChannelId === channel.id} on:click={() => setActiveChannelId('HOME', channel.id)} on:mouseover={() => hoveredChannelId = channel.id} on:mouseleave={() => hoveredChannelId = null}>
                <div class="avatar-and-name">
                    {#if channel.type === 1}
                        <img src={DiscordAssetUtils.getUserAvatarUrl(channel.recipients[0]?.id, channel.recipients[0]?.avatar, undefined, activeChannelId === channel.id || hoveredChannelId === channel.id, channel.recipients[0]?.discriminator)} alt="">
                    {:else if channel.type === 3}
                        {#if channel.icon}
                            <img src={DiscordAssetUtils.getChannelIconUrl(channel.id, channel.icon)} alt="">
                        {:else if channel.recipients.length > 1}
                            <div class="group-icon">
                                {#each channel.recipients.slice(0, 2) as recipient}
                                    <img src={DiscordAssetUtils.getUserAvatarUrl(recipient.id, recipient.avatar, undefined, activeChannelId === channel.id || hoveredChannelId === channel.id, recipient.discriminator)} alt="">
                                {/each}
                            </div>
                        {:else}
                            <img src={DefaultDMGroupAvatar} alt="">
                        {/if}
                    {/if}
                    <div class="name-status-wrapper">
                        {#if channel.type === 1}
                            <p class="name">{channel.recipients[0]?.global_name ?? channel.recipients[0]?.username}</p>
                        {:else if channel.type === 3}
                            <p class="name group">{channel.name ?? channel.recipients.map((r: any) => r.global_name ?? r.username).join(', ')}</p>
                        {/if}
                        {#if channel.type === 1}
                            <!-- <p class="status">HAHA LOL XD</p> -->
                        {:else if channel.type === 3}
                            <p class="status">{channel.recipients.length + 1} Member{channel.recipients.length + 1 > 1 ? 's' : ''}</p>
                        {/if}
                        <!-- <p class="status"></p> -->
                    </div>
                </div>
                {#if hoveredChannelId === channel.id}
                    <!-- svelte-ignore a11y_click_events_have_key_events -->
                    <!-- svelte-ignore a11y_no_static_element_interactions -->
                    <!-- svelte-ignore a11y_mouse_events_have_key_events -->
                    <div class="close-btn" on:click={() => {}} on:mouseover={() => hoveredChannelCloseButtonId = channel.id} on:mouseleave={() => hoveredChannelCloseButtonId = null}>
                        {@html DiscordIcons.withColor(DiscordIcons.ADD, hoveredChannelCloseButtonId === channel.id ? 'var(--white)' : 'var(--secondary-text)')}
                    </div>
                {/if}
            </div>
        {/each}
    </div>
</div>

<style>
    .dm-channel-list {
        display: flex;
        flex-direction: column;
        /* full sidebar width - guild list width */
        max-width: calc(375px - 75px);
        height: 100%;
    }

    .divider {
        display: flex;
        align-self: center;
        width: 95%;
        height: 1px;
        background-color: var(--border-color);
    }

    .find-a-conversation-wrapper {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 100%;
        height: 47.5px;
        padding: 6.5px;
        border-bottom: 1px solid var(--border-color);
    }

    .find-a-conversation-button {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 100%;
        height: 100%;
        background-color: var(--button-background);
        border: 1px solid var(--border-color);
        border-radius: 7.5px;
        cursor: pointer;
        transition-duration: 200ms;
    }

    .find-a-conversation-button:hover {
        background-color: var(--button-background-hover);
    }

    .find-a-conversation-button p {
        font-size: 14px;
        font-weight: 600;
    }

    .pinned-buttons {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 3px;
        width: 100%;
        padding: 6.5px 2px 6.5px 6.5px;
    }

    .pinned-button {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: start;
        gap: 10px;
        width: 100%;
        height: 40px;
        background-color: transparent;
        border-radius: 7.5px;
        padding: 5px 10px;
        cursor: pointer;
        transition-duration: 200ms;
    }
    
    .pinned-button:hover, .pinned-button.active {
        background-color: rgba(255, 255, 255, 0.035);
    }
    
    .pinned-button .icon-wrapper {
        display: flex;
        align-items: center;
        justify-content: center;
        height: 20px;
        width: 20px;
    }
    
    .pinned-button p {
        font-size: 16px;
        font-weight: 600;
        color: var(--secondary-text);
    }

    .pinned-button:hover p, .pinned-button.active p {
        color: var(--white);
    }

    .dm-channels {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
        padding: 0 2px 6.5px 6.5px;
        margin-top: 6.5px;
        overflow-y: scroll;
    }

    .dm-channels::-webkit-scrollbar {
        width: 4px;
        background-color: transparent;
    }

    .dm-channels::-webkit-scrollbar-thumb {
        background-color: var(--scrollbar-color);
        border-radius: 5px;
    }

    .create-button-wrapper {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: space-between;
        width: 100%;
        height: 15px;
        padding: 0 10px;
        margin-top: 10px;
        cursor: default;
    }
    
    .create-button-wrapper .add-icon-wrapper {
        display: flex;
        height: 17.5px;
        width: 17.5px;
        margin-bottom: 5px;
        cursor: pointer;
    }
    
    .create-button-wrapper p {
        font-size: 14px;
        font-weight: 600;
        color: var(--secondary-text);
        transition-duration: 200ms;
    }

    .create-button-wrapper:hover p {
        color: var(--white);
    }

    .dm-channel {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: space-between;
        width: 100%;
        height: 45px;
        background-color: transparent;
        border-radius: 7.5px;
        padding: 5px 10px;
        cursor: pointer;
        margin-top: 3px;
        transition-duration: 200ms;
    }

    .dm-channel:hover {
        background-color: rgba(255, 255, 255, 0.035);
    }

    .dm-channel.active {
        background-color: var(--primary-background-hover);
    }

    .dm-channel .avatar-and-name {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: start;
        gap: 10px;
        width: 90%;
        height: 100%;
    }

    .dm-channel img {
        height: 35px;
        width: 35px;
        border-radius: 50%;
    }

    .dm-channel .group-icon {
        position: relative;
        display: flex;
        align-items: center;
        justify-content: center;
        min-width: 35px;
        min-height: 35px;
    }

    .dm-channel .group-icon img {
        position: absolute;
        height: 23.5px;
        width: 23.5px;
        border-radius: 50%;
    }

    .dm-channel .group-icon img:nth-child(1) {
        margin-top: -11.5px;
        margin-left: -11.5px;
    }

    .dm-channel .group-icon img:nth-child(2) {
        margin-bottom: -11.5px;
        margin-right: -11.5px;
    }

    .dm-channel .name-status-wrapper {
        display: flex;
        flex-direction: column;
        align-items: start;
        justify-content: center;
        height: 100%;
        overflow: hidden;
    }

    .dm-channel .name-status-wrapper .name {
        font-weight: 600;
        color: var(--secondary-text);
        white-space: nowrap;
        transition-duration: 200ms;
    }
    
    .dm-channel .name-status-wrapper .name.group {
        height: 50%;
    }

    .dm-channel .name-status-wrapper .status {
        font-size: 12px;
        font-weight: 500;
        height: 50%;
        color: var(--secondary-text);
        white-space: nowrap;
        overflow: hidden;
    }

    .dm-channel .close-btn {
        display: flex;
        align-items: center;
        justify-content: center;
        transform: rotate(45deg);
        max-height: 17.5px;
        max-width: 17.5px;
    }
</style>