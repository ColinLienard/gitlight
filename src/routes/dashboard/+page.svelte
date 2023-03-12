<script lang="ts">
	import { page } from '$app/stores';
	import { EventColumn, Separator } from '~/lib/components';
	import { githubEvents } from '~/lib/stores';
	import { createEventData } from '~/lib/helpers';
	import { Check, Github, Gitlab, Mail, Pin } from '~/lib/icons';
	import type { TGithubEvent } from '~/lib/types';

	githubEvents.set(
		($page.data.githubEvents as TGithubEvent[]).map((event) => createEventData(event))
	);

	$: pinned = $githubEvents.filter((event) => event.pinned);
	$: unread = $githubEvents.filter((event) => !event.pinned && !event.read);
	$: read = $githubEvents.filter((event) => !event.pinned && event.read);
</script>

<main class="main">
	<header class="header">
		<h1 class="title">Notifications</h1>
	</header>
	<nav class="nav">
		<button class="tab selected">
			<Github />
			GitHub
		</button>
		<button class="tab">
			<Gitlab />
			GitLab
		</button>
	</nav>
	<section class="columns-container">
		<EventColumn icon={Pin} title="Pinned" events={pinned} />
		<Separator />
		<EventColumn icon={Mail} title="Unread" events={unread} />
		<Separator />
		<EventColumn icon={Check} title="Read" events={read} />
	</section>
</main>

<style lang="scss">
	.main {
		height: 100vh;
		display: flex;
		flex-direction: column;
	}

	.header {
		padding: 3rem 2rem 2rem;

		.title {
			@include typography.heading-1;
		}
	}

	.nav {
		padding: 0 2rem;
		border-bottom: 1px solid variables.$grey-3;
		display: flex;

		.tab {
			@include typography.bold;
			padding: 0.75em 1em;
			border-radius: variables.$radius variables.$radius 0 0;
			display: flex;
			align-items: center;
			gap: 0.5rem;

			:global(svg) {
				height: 1.25rem;
			}

			&.selected {
				border: 1px solid variables.$grey-3;
				border-bottom: none;
				position: relative;

				&::before {
					content: '';
					position: absolute;
					inset: auto 0 -1px 0;
					height: 1px;
					background-color: variables.$grey-1;
				}
			}

			&:not(&.selected):not(&:hover) {
				color: variables.$grey-4;
			}
		}
	}

	.columns-container {
		height: 100%;
		display: grid;
		grid-template-columns: 1fr 1px 1fr 1px 1fr;
		gap: 1.5rem;
		padding: 2rem;
		overflow: hidden;
	}
</style>
