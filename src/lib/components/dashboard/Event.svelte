<script lang="ts">
	import { getHex } from '~/lib/helpers';
	import { Check, ExternalLink, Mail, Pin, Unpin } from '~/lib/icons';
	import type { TEvent } from '~/lib/types';
	import { Button, Tooltip } from '~/lib/components';
	import { githubEvents } from '~/lib/stores';

	export let data: TEvent;

	let {
		id,
		read,
		pinned,
		description,
		icon,
		iconColor,
		repo: fullRepo,
		time,
		title,
		url,
		labels,
		number
	} = data;
	let [owner, repo] = fullRepo.split('/');

	function handleToggle(key: 'read' | 'pinned') {
		return () => {
			githubEvents.update((previous) =>
				previous.map((event) => (event.id === id ? { ...event, [key]: !event[key] } : event))
			);
			if (pinned) {
				read = !read;
			}
		};
	}
</script>

<div class="event">
	<div class="top">
		<p class="repo">{owner}/<span class="bold">{repo}</span></p>
		<p class="time">{time}</p>
	</div>
	<p class="description">
		{#each description as item}
			{#if typeof item === 'string'}
				<span class="subtle">{item}</span>
			{:else}
				{item.text}
				<span class="image-container">
					{#if item.icon}
						<span style:color={getHex(item.iconColor)}>
							<svelte:component this={item.icon} />
						</span>
					{/if}
					{#if item.image}
						<img class="image" src={item.image} alt="" />
					{/if}
				</span>
			{/if}
		{/each}
	</p>
	<div class="main">
		<span class="icon-container" style:color={getHex(iconColor)}>
			<svelte:component this={icon} />
		</span>
		<h3 class="title">{title}</h3>
		{#if number}
			<span class="number">#{number}</span>
		{/if}
	</div>
	{#if labels && labels.length}
		<ul class="labels">
			{#each labels as label}
				<li class="label" style:color={`#${label.color}`}>{label.name}</li>
			{/each}
		</ul>
	{/if}
	<div class="over">
		<Tooltip content="Mark as {read ? 'un' : ''}read" position="left">
			<Button type={read ? 'secondary' : 'primary'} small on:click={handleToggle('read')}>
				{#if read}
					<Mail />
				{:else}
					<Check />
				{/if}
			</Button>
		</Tooltip>
		{#if url}
			<Tooltip content="Open in GitHub" position="left">
				<Button type="secondary" small href={url} external><ExternalLink /></Button>
			</Tooltip>
		{/if}
		<Tooltip content={pinned ? 'Unpin' : 'Pin'} position="left">
			<Button type="secondary" small on:click={handleToggle('pinned')}>
				{#if pinned}
					<Unpin />
				{:else}
					<Pin />
				{/if}
			</Button>
		</Tooltip>
	</div>
</div>

<style lang="scss">
	.event {
		background-color: variables.$grey-2;
		border: 1px solid variables.$grey-3;
		border-radius: variables.$radius;
		padding: 1rem;
		position: relative;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;

		&:not(&:hover) {
			.over {
				opacity: 0;
			}
		}
	}

	.top {
		@include typography.small;
		display: flex;
		justify-content: space-between;
		color: variables.$grey-4;

		.repo .bold {
			@include typography.bold;
		}
	}

	.description {
		.subtle {
			color: variables.$grey-4;
		}

		:global(svg),
		.image {
			display: inline-block;
			width: 1.25rem;
			height: 1.25rem;
			translate: 0 0.25rem;
		}

		.image {
			border-radius: 50%;
		}
	}

	.main {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		width: 100%;

		.icon-container {
			flex: 0 0 2rem;

			:global(svg) {
				width: 2rem;
				height: 2rem;
			}
		}

		.title {
			@include typography.bold;
			flex: 0 1 auto;
			white-space: nowrap;
			overflow-x: hidden;
			text-overflow: ellipsis;
			padding-bottom: 2px;
		}

		.number {
			color: variables.$blue-3;
		}
	}

	.labels {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;

		.label {
			@include typography.small;
			padding: 0.25rem 0.5rem;
			border-radius: variables.$radius;
			border: 1px solid;
			position: relative;

			&::before {
				content: '';
				position: absolute;
				inset: 0;
				border-radius: variables.$radius;
				background-color: currentColor;
				opacity: 0.1;
			}
		}
	}

	.over {
		position: absolute;
		inset: 0 0 0 auto;
		width: 12rem;
		background-image: linear-gradient(to right, transparent, variables.$grey-1);
		border-radius: 0 calc(variables.$radius - 1px) calc(variables.$radius - 1px) 0;
		display: flex;
		flex-direction: column;
		padding: 0.5rem;
		align-items: end;
		justify-content: start;
		gap: 0.5rem;
		transition: opacity variables.$transition;
	}
</style>
