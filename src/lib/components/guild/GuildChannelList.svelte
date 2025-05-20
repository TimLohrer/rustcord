<script lang="ts">
	import { PermissionFlags, PermissionUtils } from './../../utils/permissionUtils';
    import { ACTIVE_CHANNEL_ID, ACTIVE_GUILD_ID, GUILDS, setActiveChannelId, VOICE_STATE } from "$lib/stores/stateStore";
    import { ChannelType } from "$lib/types/channel";
    import { DiscordAssetUtils } from "$lib/utils/discordAssetUtils";
    import { DiscordIcons } from "$lib/utils/iconUtils";
  import { joinVoiceChannel, leaveVoiceChannel } from '$lib/api/gateway';

    let hoveredChannelId: string | null = null;

    let closedCategories: Record<string, string[]> = {};

    let scrollIndex = 0;

    $: guild = $GUILDS.find(g => g.id === $ACTIVE_GUILD_ID);

    $: activeChannelId = $ACTIVE_CHANNEL_ID[guild.id] ?? 'BROWSE_CHANNELS';

    $: standaloneChannels = guild.channels
        ?.filter((c: any) => c.parent_id === null && c.type !== 4)
        .sort((a: any, b: any) => a.position - b.position) ?? [];

    $: categoryChannels = guild.channels
        ?.filter((c: any) => c.type === 4)
        .sort((a: any, b: any) => a.position - b.position) ?? [];

    $: categoryChildChannels = (() => {
        const categories: { [key: string]: any[] } = {};
        for (const category of categoryChannels) {
            categories[category.id] = guild.channels
                ?.filter((c: any) => c.parent_id === category.id && c.type !== 4)
                .sort((a: any, b: any) => b.position - a.position)
                .sort((c: any) => c.type !== 2 ? -1 : 1) ?? [];
        }
        return categories;
    })();

    async function handleChannelClick(channelType: ChannelType, channelId: string) {
        setActiveChannelId(guild.id, channelId);

        if (channelType === ChannelType.VOICE && ($VOICE_STATE === null || $VOICE_STATE?.channelId !== channelId)) {
            await joinVoiceChannel(guild.id, channelId);
        } else if (channelType === ChannelType.VOICE && $VOICE_STATE) {
            await leaveVoiceChannel(guild.id);
        }
    }
</script>

<div class="guild-channel-list">
    <div class="banner-header">
        <div class="guild-name">
            {#if guild.features.includes('COMMUNITY') || guild.premium_tier > 0}
                <div class="badge-wrapper">
                    <div class="verify-icon-wrapper">
                        {@html DiscordIcons.withColor(DiscordIcons.VERIFYED_BADGE, guild.features.includes('PARTNERED') ? 'var(--blurple)' : guild.features.includes('VERIFIED') ? 'var(--green)' : guild.features.includes('COMMUNITY') && guild.premium_tier > 0 ? 'url(#nitro)' : guild.features.includes('COMMUNITY') ? 'var(--white)' : 'var(--primary-background)')}
                    </div>
                    <div class="verify-icon-detail-wrapper">
                        {#if guild.features.includes('PARTNERED')}
                            {@html DiscordIcons.withColor(DiscordIcons.PARTNER, 'var(--white)')}
                        {:else if guild.features.includes('VERIFIED')}
                            {@html DiscordIcons.withColor(DiscordIcons.CHECKMARK, 'var(--white)')}
                        {:else if guild.features.includes('DISCOVERABLE')}
                            {@html DiscordIcons.withColor(DiscordIcons.GLOBE, guild.premium_tier > 0 ? 'var(--white)' : 'var(--primary-background)')}
                        {:else if guild.features.includes('COMMUNITY')}
                            {@html DiscordIcons.withColor(DiscordIcons.HOME, guild.premium_tier > 0 ? 'var(--white)' : 'var(--primary-background)')}
                        {:else if guild.premium_subscription_count > 0}
                            {@html DiscordIcons.withColor(DiscordIcons.NITRO_GEM, 'var(--white)')}
                        {/if}
                    </div>
                </div>
            {/if}
            <p class="name">{guild.name}</p>
            <div class="icon-wrapper">
                {@html DiscordIcons.withColor(DiscordIcons.CHEVRON, 'var(--white)')}
            </div>
        </div>
        {#if guild.banner}
            <img src={DiscordAssetUtils.getGuildBannerUrl(guild.id, guild.banner, 512, guild.features.includes('ANIMATED_BANNER'))} alt="Guild Banner" class="banner" style={`opacity: ${scrollIndex <= 0 ? 1 : 1 - (scrollIndex / 100) };`} />
        {/if}
    </div>
    <div class="guild-channels" on:scroll={(e) => scrollIndex = (e.target as HTMLElement).scrollTop} style={`padding-top: ${guild.banner ? 125 : 0}px;`}>
        <div class="scroll-background" style={`0px 2px 6.5px 6.5px;`}>
            {#if PermissionUtils.hasAnyPermission(guild.permissions, [PermissionFlags.ADMINISTRATOR, PermissionFlags.MANAGE_GUILD, PermissionFlags.CREATE_EVENTS])}
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <div class="guild-channel" on:click={() => setActiveChannelId(guild.id, 'EVENTS')} class:active={activeChannelId === 'EVENTS'}>
                    <div class="icon-wrapper">
                        {@html DiscordIcons.withColor(DiscordIcons.CALENDER, activeChannelId === 'EVENTS' ? 'var(--white)' : 'var(--secondary-text)')}
                    </div>
                    <p class="name">Events</p>
                </div>
            {/if}
            {#if guild.features.includes('COMMUNITY')}
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <div class="guild-channel" on:click={() => setActiveChannelId(guild.id, 'BROWSE_CHANNELS')} class:active={activeChannelId === 'BROWSE_CHANNELS'}>
                    <div class="icon-wrapper">
                        {@html DiscordIcons.withColor(DiscordIcons.BROWSE, activeChannelId === 'BROWSE_CHANNELS' ? 'var(--white)' : 'var(--secondary-text)')}
                    </div>
                    <p class="name">Browse Channels</p>
                </div>
            {/if}
            {#if PermissionUtils.hasAnyPermission(guild.permissions, [PermissionFlags.ADMINISTRATOR, PermissionFlags.MANAGE_GUILD, PermissionFlags.MANAGE_ROLES, PermissionFlags.MANAGE_NICKNAMES, PermissionFlags.MODERATE_MEMBERS, PermissionFlags.KICK_MEMBERS, PermissionFlags.BAN_MEMBERS])}
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <div class="guild-channel" on:click={() => setActiveChannelId(guild.id, 'MEMBERS')} class:active={activeChannelId === 'MEMBERS'}>
                    <div class="icon-wrapper">
                        {@html DiscordIcons.withColor(DiscordIcons.MEMBERS, activeChannelId === 'MEMBERS' ? 'var(--white)' : 'var(--secondary-text)')}
                    </div>
                    <p class="name">Members</p>
                </div>
            {/if}
            <!-- svelte-ignore element_invalid_self_closing_tag -->
            <div class="divider" />
            {#each standaloneChannels as channel}
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_mouse_events_have_key_events -->
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <div class="guild-channel" on:click={() => handleChannelClick(channel.type as ChannelType, channel.id)} class:active={activeChannelId === channel.id} on:mouseover={() => hoveredChannelId = channel.id} on:mouseleave={() => hoveredChannelId = null}>
                    {#if channel.type === ChannelType.TEXT}
                        <div class="icon-wrapper">
                            {@html DiscordIcons.withColor(channel.id === guild.rules_channel_id ? DiscordIcons.RULES : DiscordIcons.TEXT_CHANNEL, activeChannelId === channel.id ? 'var(--white)' : 'var(--secondary-text)')}
                        </div>
                    {:else if channel.type === ChannelType.VOICE}
                        <div class="icon-wrapper">
                            {@html DiscordIcons.withColor(DiscordIcons.VOICE, activeChannelId === channel.id ? 'var(--white)' : 'var(--secondary-text)')}
                        </div>
                    {:else if channel.type === ChannelType.FORUM}
                        <div class="icon-wrapper">
                            {@html DiscordIcons.withColor(DiscordIcons.FORUM, activeChannelId === channel.id ? 'var(--white)' : 'var(--secondary-text)')}
                        </div>
                    {:else if channel.type === ChannelType.NEWS}
                        <div class="icon-wrapper">
                            {@html DiscordIcons.withColor(DiscordIcons.ANNOUNCEMENT, activeChannelId === channel.id ? 'var(--white)' : 'var(--secondary-text)')}
                        </div>
                    {:else if channel.type === ChannelType.STAGE_VOICE}
                        <div class="icon-wrapper">
                            {@html DiscordIcons.withColor(DiscordIcons.STAGE, activeChannelId === channel.id ? 'var(--white)' : 'var(--secondary-text)')}
                        </div>
                    {/if}
                    <p class="name">{channel.name}</p>
                </div>
            {/each}
            {#each categoryChannels as category}
                <div class="category">
                    <!-- svelte-ignore a11y_no_static_element_interactions -->
                    <!-- svelte-ignore a11y_click_events_have_key_events -->
                    <!-- svelte-ignore a11y_mouse_events_have_key_events -->
                    <div class="header" on:click={() => closedCategories[guild.id]?.includes(category.id) ? closedCategories[guild.id] = closedCategories[guild.id]?.filter(c => c !== category.id) : closedCategories[guild.id] = [...closedCategories[guild.id] ?? [], category.id]} on:mouseover={() => hoveredChannelId = category.id} on:mouseleave={() => hoveredChannelId = null}>
                        <p class="name">{category.name}</p>
                        <div class="icon-wrapper" class:open={!closedCategories[guild.id]?.includes(category.id)}>
                            {@html DiscordIcons.withColor(DiscordIcons.CHEVRON, hoveredChannelId === category.id ? 'var(--white)' : 'var(--secondary-text)')}
                        </div>
                    </div>
                    <div class="children" class:open={!closedCategories[guild.id]?.includes(category.id)}>
                        {#each categoryChildChannels[category.id] as channel}
                            <!-- svelte-ignore a11y_click_events_have_key_events -->
                            <!-- svelte-ignore a11y_mouse_events_have_key_events -->
                            <!-- svelte-ignore a11y_no_static_element_interactions -->
                            <div class="guild-channel inside-category" on:click={() => handleChannelClick(channel.type as ChannelType, channel.id)} class:active={activeChannelId === channel.id} on:mouseover={() => hoveredChannelId = channel.id} on:mouseleave={() => hoveredChannelId = null}>
                                {#if channel.type === ChannelType.TEXT}
                                    <div class="icon-wrapper">
                                        {@html DiscordIcons.withColor(channel.id === guild.rules_channel_id ? DiscordIcons.RULES : DiscordIcons.TEXT_CHANNEL, activeChannelId === channel.id ? 'var(--white)' : 'var(--secondary-text)')}
                                    </div>
                                {:else if channel.type === ChannelType.VOICE}
                                    <div class="icon-wrapper">
                                        {@html DiscordIcons.withColor(DiscordIcons.VOICE, activeChannelId === channel.id ? 'var(--white)' : 'var(--secondary-text)')}
                                    </div>
                                {:else if channel.type === ChannelType.FORUM}
                                    <div class="icon-wrapper">
                                        {@html DiscordIcons.withColor(DiscordIcons.FORUM, activeChannelId === channel.id ? 'var(--white)' : 'var(--secondary-text)')}
                                    </div>
                                {:else if channel.type === ChannelType.NEWS}
                                    <div class="icon-wrapper">
                                        {@html DiscordIcons.withColor(DiscordIcons.ANNOUNCEMENT, activeChannelId === channel.id ? 'var(--white)' : 'var(--secondary-text)')}
                                    </div>
                                {:else if channel.type === ChannelType.STAGE_VOICE}
                                    <div class="icon-wrapper">
                                        {@html DiscordIcons.withColor(DiscordIcons.STAGE, activeChannelId === channel.id ? 'var(--white)' : 'var(--secondary-text)')}
                                    </div>
                                {/if}
                                <p class="name">{channel.name}</p>
                            </div>
                        {/each}
                    </div>
                </div>
            {/each}
        </div>
    </div>
</div>

<style>
    .guild-channel-list {
        position: relative;
        display: flex;
        flex-direction: column;
        /* full width - guild sidebar */
        max-width: 100%;
        height: 100%;
    }

    .divider {
        display: flex;
        align-self: center;
        width: 95%;
        min-height: 1px;
        margin: 7.5px 0;
        background-color: var(--border-color);
    }

    .banner-header {
        display: flex;
        width: 100%;
        border-top-left-radius: 10px;
    }

    .banner-header .banner {
        display: flex;
        width: 100%;
        height: 175px;
        position: absolute;
        border-top-left-radius: 10px;
        mask-image: linear-gradient(to bottom, rgba(0, 0, 0, 0.5) 0%, rgba(0, 0, 0, 1) 100%);
    }

    .banner-header .guild-name {
        display: flex;
        height: 50px;
        z-index: 2;
        padding: 10px;
        align-items: center;
        backdrop-filter: blur(1px) brightness(0.975);
        border-top-left-radius: 10px;
        gap: 5px;
        width: 100%;
        overflow: hidden;
        cursor: pointer;
        transition-duration: 200ms;
    }

    .banner-header .guild-name:hover {
        backdrop-filter: blur(15px) brightness(0.9);
    }

    .banner-header .guild-name .badge-wrapper {
        display: flex;
        width: 22.5px;
        height: 22.5px;
        align-items: center;
        justify-content: center;
    }

    .banner-header .guild-name .badge-wrapper .verify-icon-wrapper {
        display: flex;
        max-width: 20px;
        max-height: 20px;
        align-items: center;
        justify-content: center;
    }

    .banner-header .guild-name .badge-wrapper .verify-icon-detail-wrapper {
        position: absolute;
        display: flex;
        width: 12.5px;
        height: 12.5px;
        align-items: center;
        justify-content: center;
        z-index: 2;
    }

    .banner-header .guild-name .name {
        font-size: 18px;
        font-weight: 700;
        color: white;
        white-space: nowrap;
    }

    .banner-header .guild-name .icon-wrapper {
        display: flex;
        width: 20px;
        height: 20px;
        align-items: center;
        justify-content: center;
        margin-left: auto;
        transform: rotate(90deg);
    }

    .guild-channels {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
        overflow-y: scroll;
        overflow-x: hidden;
        z-index: 1;
    }

    .guild-channels .scroll-background {
        display: flex;
        flex-direction: column;
        background-color: var(--secondary-background);
        padding: 0 3.5px 6.5px 6.5px;
        width: 100%;
        height: 100%;
        gap: 3.5px;
    }

    .guild-channels::-webkit-scrollbar {
        width: 4px;
        background: transparent;
    }
    
    .guild-channels::-webkit-scrollbar-thumb {
        background-color: var(--scrollbar-color);
        border-radius: 5px;
    }
    
    .guild-channel {
        display: flex;
        width: 100%;
        height: 32.5px;
        align-items: center;
        gap: 10px;
        padding: 5px 10px;
        border-radius: 5px;
        cursor: pointer;
        transition-duration: 200ms;
    }
    
    .guild-channel:not(.inside-category):nth-child(1) {
        margin-top: 10px;
    }

    .guild-channel:hover, .guild-channel.active {
        background-color: rgba(255, 255, 255, 0.035);
    }

    .guild-channel:hover p, .guild-channel.active p {
        color: var(--white);
    }

    .guild-channel .icon-wrapper {
        display: flex;
        width: 20px;
        height: 20px;
        align-items: center;
        justify-content: center;
    }

    .guild-channel .name {
        font-size: 16px;
        font-weight: 600;
        color: var(--secondary-text);
        white-space: nowrap;
    }

    .category {
        display: flex;
        flex-direction: column;
        width: 100%;
        margin-top: 20px;
    }

    .category .header {
        display: flex;
        width: 100%;
        height: 20px;
        align-items: center;
        padding: 5px 10px;
        cursor: pointer;
        gap: 5px;
        transition-duration: 200ms;
    }

    .category .header:hover .name {
        color: var(--white);
    }

    .category .header .name {
        font-size: 14px;
        font-weight: 600;
        color: var(--secondary-text);
        white-space: nowrap;
    }

    .category .header .icon-wrapper {
        display: flex;
        width: 15px;
        height: 15px;
        align-items: center;
        justify-content: center;
        transform: rotate(0deg);
        transition-duration: 200ms;
    }

    .category .header .icon-wrapper.open {
        transform: rotate(90deg);
    }

    .category .children {
        display: flex;
        flex-direction: column;
        width: 100%;
        gap: 3.5px;
        margin-top: 5px;
        display: none;
        transition-duration: 200ms;
    }

    .category .children.open {
        display: flex;
    }
</style>