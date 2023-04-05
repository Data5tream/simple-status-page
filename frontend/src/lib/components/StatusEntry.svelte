<script lang="ts">
	import type { Watchpoint } from '../dataprovider';
	import StatusIcon from './StatusIcon.svelte';

	export let data: Watchpoint;
</script>

<div class="watchpoint">
	<h4>{data.watchpoint.name}</h4>
	<div class="content">
		Status: <StatusIcon status={data.status} />
		{data.status}<br />
		URL: <a href={data.watchpoint.url} class="url">{data.watchpoint.url}</a>
	</div>
</div>

<style>
	.watchpoint {
		flex: 1 0 0;
		border: 1px solid var(--c-black);
		min-width: 300px;
	}

	h4 {
		background: var(--c-black);
		color: var(--c-white);
		letter-spacing: 0.15em;
		margin: 0;
		padding: 8px;
	}

	.content {
		padding: 16px 8px 8px;
	}

	.url {
		font-family: monospace;
	}
</style>
