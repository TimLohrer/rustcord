<script lang="ts">
	import GuildChannelList from './../lib/components/guild/GuildChannelList.svelte';
	import DMChannelList from './../lib/components/home/DMChannelList.svelte';
	import TitleBar from './../lib/components/TitleBar.svelte';
	import GuildSidebar from './../lib/components/GuildSidebar.svelte';
	import ActiveUserCard from './../lib/components/home/ActiveUserCard.svelte';
	import { loadDebugToken, token_login } from "$lib/api/login";
	import { ACTIVE_GUILD_ID, ACTIVE_USER, TOKEN, VOICE_SOUND_STATE, VOICE_STATE } from "$lib/stores/stateStore";
	import { onMount } from "svelte";
	import { connectToGateway, handleGatewayEvent } from '$lib/api/gateway';
	import { listen } from '@tauri-apps/api/event';

	onMount(async () => {
		listen('GATEWAY_EVENT', handleGatewayEvent);
		
		await loadDebugToken();

		TOKEN.subscribe(async (newToken) => {
			if (newToken) {
				await connectToGateway();
			}
		});

	});
</script>

<div class="rustcord">
	{#if $ACTIVE_USER}
		<div class="app-root">
			<div class="title-bar">
				<TitleBar />
			</div>
			<div class="app-content">
				<div class="guild-and-dm-channel-list-wrapper">
					<!-- /* Incomming dynamic CSS: full height - active user card height - title bar height */ -->
					<div class="lists-wrapper" style={`height: calc(100% - ${$VOICE_STATE !== null ? 215 : 75}px - 45px);`}>
						<div class="guild-list">
							<GuildSidebar />
						</div>
						{#if $ACTIVE_GUILD_ID === 'HOME'}
							<div class="dm-channels">
								<DMChannelList />
							</div>
						{:else}
							<div class="guild-channels">
								<GuildChannelList />
							</div>
						{/if}
					</div>
					<div class="current-user-card" style={`height: ${$VOICE_STATE !== null ? 215 : 75}px;`}>
						<ActiveUserCard />
					</div>
				</div>
				<div class="active-channel">

				</div>
			</div>
		</div>
	{:else}
		<input type="text" placeholder="Your Token" on:change={(e) => { const target = e.target as HTMLInputElement | null; if (target) token_login(target.value); }}>
	{/if}
</div>

<style>
	.rustcord, .app-root {
		display: flex;
		flex-direction: column;
		height: 100%;
		width: 100%;
		overflow: hidden;
		background-color: var(--secondary-background);
	}

	.title-bar {
		display: flex;
		min-height: 35px;
		width: 100%;
	}

	.app-content {
		display: flex;
		width: 100%;
		height: 100%;
		background-color: transparent;
	}

	.guild-and-dm-channel-list-wrapper {
		display: flex;
		flex-direction: column;
		width: 400px;
		height: 100%;
	}

	.lists-wrapper {
		display: flex;
		flex-direction: row;
	}

	.guild-list {
		display: flex;
		flex-direction: column;
		min-width: 75px;
		height: 100%;
	}

	.guild-channels {
		display: flex;
		flex-direction: column;
		width: calc(100% - 75px);
		border-top-left-radius: 10px;
		border-left: 1px solid var(--border-color);
		border-top: 1px solid var(--border-color);
	}

	.dm-channels {
		display: flex;
		flex-direction: column;
		width: 100%;
		border-top-left-radius: 10px;
		border-left: 1px solid var(--border-color);
		border-top: 1px solid var(--border-color);
	}

	.current-user-card {
		display: flex;
		/* full width - 10px both sides margin */
		width: calc(100% - 20px);
		border: 1px solid var(--border-color);
		border-radius: 10px;
		margin: 0 10px 10px 10px;
	}

	.active-channel {
		display: flex;
		flex-direction: column;
		width: 100%;
		border-top: 1px solid var(--border-color);
	}
</style>