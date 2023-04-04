<script lang="ts">
	import type { Watchpoint } from '../dataprovider';

	export let data: Watchpoint;
</script>

<div class="watchpoint">
	<h4>{data.watchpoint.name}</h4>
	Status: {data.status}<br />
	URL: <a href={data.watchpoint.url} class="url">{data.watchpoint.url}</a>
</div>

<style>
	.watchpoint {
		flex: 1 0 0;
		border: 1px solid #343434;
		padding: 8px;
		min-width: 300px;
	}

	.url {
		font-family: monospace;
	}
</style>
