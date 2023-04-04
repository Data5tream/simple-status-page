<script lang="ts">
	import type { Watchpoint } from '$lib/dataprovider';
	import StatusEntry from '$lib/components/StatusEntry.svelte';

	export let data: Array<Watchpoint> | undefined;
</script>

<div class="list-container">
	{#if data}
		{#each data as entry}
			<StatusEntry bind:data={entry} />
		{/each}
	{:else}
		<span>No data to show</span>
	{/if}
</div>

<style>
	.list-container {
		display: flex;
		flex-wrap: wrap;
		gap: 16px;
	}
</style>
