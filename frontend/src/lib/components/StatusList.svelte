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

<style lang='scss'>
	.list-container {
		display: grid;
		grid-template-columns: 1fr;
		gap: 16px;

    @media (min-width: 768px) {
      grid-template-columns: repeat(2, 1fr);
		}

    @media (min-width: 1280px) {
      grid-template-columns: repeat(3, 1fr);
    }
	}
</style>
