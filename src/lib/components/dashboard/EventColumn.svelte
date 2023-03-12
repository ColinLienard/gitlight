<script lang="ts">
	import type { ComponentType } from 'svelte';
	import type { TEvent } from '~/lib/types';
	import Event from './Event.svelte';

	export let icon: ComponentType;
	export let title: string;
	export let events: TEvent[];
</script>

<ul class="column">
	<li class="column-header">
		<svelte:component this={icon} />
		<h3 class="title">{title}</h3>
		<p class="number">{events.length}</p>
	</li>
	{#each events as event (event.id)}
		<li>
			<Event data={event} />
		</li>
	{/each}
</ul>

<style lang="scss">
	.column {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		overflow: auto;

		&::-webkit-scrollbar {
			display: none;
		}

		.column-header {
			display: flex;
			align-items: center;
			gap: 0.5rem;

			:global(svg) {
				height: 1.25rem;
			}

			.title {
				@include typography.bold;
			}

			.number {
				color: variables.$grey-4;
			}
		}
	}
</style>
