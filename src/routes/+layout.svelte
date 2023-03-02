<script lang="ts">
	import { page } from '$app/stores';
	import type { Session } from '@auth/core/types';
	import { onMount } from 'svelte';
	import { signIn } from '@auth/sveltekit/client';
	import { invoke } from '@tauri-apps/api/tauri';

	type Notification = {
		repository: {
			full_name: string;
		};
		subject: {
			title: string;
			type: string;
		};
		unread: boolean;
	};

	let notifications: Notification[] = [];

	onMount(async () => {
		const session = $page.data.session as (Session & { accessToken: string }) | null;
		if (!session) {
			return;
		}
		const response = await fetch('https://api.github.com/notifications?all=true', {
			headers: {
				Accept: 'application/vnd.github+json',
				Authorization: `Bearer ${session.accessToken}`
			}
		});
		const data = await response.json();
		notifications = data;

		document.addEventListener('readystatechange', async (event) => {
			await invoke('github_auth_ready');
		});
	});

	async function signInToGithub() {
		await invoke('github_auth_flow');
	}
</script>

<div>
	<header>
		<div class="signedInStatus">
			<p class="nojs-show loaded">
				{#if $page.data.session}
					{#if $page.data.session.user?.image}
						<span style="background-image: url('{$page.data.session.user.image}')" class="avatar" />
					{/if}
					<span class="signedInText">
						<small>Signed in as</small><br />
						<strong>{$page.data.session.user?.email ?? $page.data.session.user?.name}</strong>
					</span>
					<a href="/auth/signout" class="button" data-sveltekit-preload-data="off">Sign out</a>
				{:else}
					<button on:click={signInToGithub}>signin 2</button>
					<span class="notSignedInText">You are not signed in</span>
					<a href="/auth/signin" class="buttonPrimary" data-sveltekit-preload-data="off">Sign in</a>
				{/if}
			</p>
		</div>
		<nav>
			<ul class="navItems">
				<li class="navItem"><a href="/">Home</a></li>
				<li class="navItem"><a href="/protected">Protected</a></li>
			</ul>
		</nav>
	</header>
	<slot />
	<ul>
		{#each notifications as notification}
			<li>
				<h2>
					{#if notification.unread}
						<span>🚨</span>
					{/if}
					{notification.subject.title}
				</h2>
				<p>{notification.subject.type}</p>
				<small>{notification.repository.full_name}</small>
			</li>
		{/each}
	</ul>
</div>

<style>
	:global(body) {
		font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto,
			'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji',
			'Segoe UI Symbol', 'Noto Color Emoji';
		padding: 0 1rem 1rem 1rem;
		max-width: 680px;
		margin: 0 auto;
		background: #fff;
		color: #333;
	}
	:global(li),
	:global(p) {
		line-height: 1.5rem;
	}
	:global(a) {
		font-weight: 500;
	}
	:global(hr) {
		border: 1px solid #ddd;
	}
	:global(iframe) {
		background: #ccc;
		border: 1px solid #ccc;
		height: 10rem;
		width: 100%;
		border-radius: 0.5rem;
		filter: invert(1);
	}

	.nojs-show {
		opacity: 1;
		top: 0;
	}
	.signedInStatus {
		display: block;
		min-height: 4rem;
		width: 100%;
	}
	.loaded {
		position: relative;
		top: 0;
		opacity: 1;
		overflow: hidden;
		border-radius: 0 0 0.6rem 0.6rem;
		padding: 0.6rem 1rem;
		margin: 0;
		background-color: rgba(0, 0, 0, 0.05);
		transition: all 0.2s ease-in;
	}
	.signedInText,
	.notSignedInText {
		position: absolute;
		padding-top: 0.8rem;
		left: 1rem;
		right: 6.5rem;
		white-space: nowrap;
		text-overflow: ellipsis;
		overflow: hidden;
		display: inherit;
		z-index: 1;
		line-height: 1.3rem;
	}
	.signedInText {
		padding-top: 0rem;
		left: 4.6rem;
	}
	.avatar {
		border-radius: 2rem;
		float: left;
		height: 2.8rem;
		width: 2.8rem;
		background-color: white;
		background-size: cover;
		background-repeat: no-repeat;
	}
	.button,
	.buttonPrimary {
		float: right;
		margin-right: -0.4rem;
		font-weight: 500;
		border-radius: 0.3rem;
		cursor: pointer;
		font-size: 1rem;
		line-height: 1.4rem;
		padding: 0.7rem 0.8rem;
		position: relative;
		z-index: 10;
		background-color: transparent;
		color: #555;
	}
	.buttonPrimary {
		background-color: #346df1;
		border-color: #346df1;
		color: #fff;
		text-decoration: none;
		padding: 0.7rem 1.4rem;
	}
	.buttonPrimary:hover {
		box-shadow: inset 0 0 5rem rgba(0, 0, 0, 0.2);
	}
	.navItems {
		margin-bottom: 2rem;
		padding: 0;
		list-style: none;
	}
	.navItem {
		display: inline-block;
		margin-right: 1rem;
	}
</style>
