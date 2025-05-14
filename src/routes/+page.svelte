<script lang="ts">
	import TitleBar from './../lib/components/TitleBar.svelte';
	import GuildSidebar from './../lib/components/GuildSidebar.svelte';
	import ActiveUserCard from './../lib/components/home/ActiveUserCard.svelte';
	import { loadDebugToken, token_login } from "$lib/api/login";
  	import { loadSettings } from "$lib/api/settings";
	import { ACTIVE_GUILD_ID, ACTIVE_USER, TOKEN } from "$lib/stores/stateStore";
	import { onMount } from "svelte";
  	import { loadGuilds } from '$lib/api/guilds';
  	import { loadDMChannels } from '$lib/api/channels';

	onMount(async () => {
		await loadDebugToken();

		TOKEN.subscribe(async (newToken) => {
			if (newToken) {
				await token_login($TOKEN);
				await loadSettings();
				await loadDMChannels();
				await loadGuilds();
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
					<div class="lists-wrapper">
						<div class="guild-list">
							<GuildSidebar />
						</div>
						{#if $ACTIVE_GUILD_ID === 'HOME'}
							<div class="dm-channels">
			
							</div>
						{:else}
							<div class="guild-channels">
								
							</div>
						{/if}
					</div>
					<div class="current-user-card">
						<ActiveUserCard />
					</div>
				</div>
				<div class="active-channel">

				</div>
			</div>
		</div>
	{:else}
		<input type="text" placeholder="Your Token" on:change={(e) => token_login(e.target.value)}>
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
		width: max-content;
		height: 100%;
	}

	.lists-wrapper {
		display: flex;
		flex-direction: row;
		height: calc(100% - 75px - 45px);
	}

	.guild-list {
		display: flex;
		flex-direction: column;
		width: 85px;
		height: 100%;
	}

	.guild-channels {
		display: flex;
		flex-direction: column;
		width: 385px;
		border-top-left-radius: 20px;
		border-left: 1px solid var(--border-color);
		border-top: 1px solid var(--border-color);
	}

	.dm-channels {
		display: flex;
		flex-direction: column;
		width: 385px;
		border-top-left-radius: 20px;
		border-left: 1px solid var(--border-color);
		border-top: 1px solid var(--border-color);
	}

	.current-user-card {
		display: flex;
		width: calc(100% - 20px);
		height: 75px;
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