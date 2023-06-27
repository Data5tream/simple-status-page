<script lang="ts">
	import type { StatusData } from '$lib/dataprovider';
	import StatusList from '$lib/components/StatusList.svelte';
	import PageFooter from '$lib/components/PageFooter.svelte';
	import '$lib/main.css';

	export let data: { status: StatusData };
</script>

<main>
	<h1>Simple Status Page</h1>
	{#if data.status && !data.status.is_valid}
		<span class="error">Error loading status data :(</span>
	{:else}
		<StatusList bind:data={data.status.watchpoints} />
	{/if}
</main>
<PageFooter />

<style lang="scss">
	main {
		max-width: 1400px;
		margin: 0 16px;

		@media screen and (min-width: 1432px) {
			width: 100%;
			margin: 0 auto;
    }
	}

	h1 {
		display: inline-block;
		background: var(--c-black);
		color: var(--c-white);
		padding: 8px 16px 8px 8px;
		font-family: monospace;

		&:after {
			content: '_';
			animation: blink 2s infinite;
		}
	}

	.error {
		color: var(--c-red);
		font-size: 64px;
	}
</style>
