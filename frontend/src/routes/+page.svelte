<script lang="ts">
	import type { StatusData } from '$lib/dataprovider';
	import StatusList from '$lib/components/StatusList.svelte';

	export let data: { status: StatusData };
</script>

<main>
	<h1>Simple Status Page</h1>
	{#if !data.status.is_valid}
		<span class="error">Error loading status data :(</span>
	{:else}
		<StatusList bind:data={data.status.watchpoints} />
	{/if}
</main>

<style>
	main {
		max-width: 1400px;
		margin: 0 auto;
	}
	.error {
		color: #870606;
		font-size: 64px;
	}
</style>
