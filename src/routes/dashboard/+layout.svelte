<script lang="ts">
	import { signOut } from '@auth/sveltekit/client';
</script>

<button on:click={signOut}>Sign out</button>
<slot />

<style lang="scss">
	button {
		position: absolute;
		inset: 3rem 2rem auto auto;
	}
</style>
