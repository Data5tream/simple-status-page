<script lang="ts">
  import type { Watchpoint } from '../dataprovider';
  import StatusIcon from './StatusIcon.svelte';

  export let data: Watchpoint;

  let statusMsg = '';
  $: if (data) {
    if (data.status === 200) {
      statusMsg = 'OK (200)';
    } else if (data.status === 404) {
      statusMsg = `Not found (${data.status})`;
    } else if (data.status < 600) {
      statusMsg = `${data.status}`;
    } else if (data.status === 600) {
      statusMsg = 'Invalid DNS';
    } else if (data.status === 601) {
      statusMsg = 'TLS validation error';
    } else if (data.status === 604) {
      statusMsg = 'Keyword not found';
    } else if (data.status === 610) {
      statusMsg = 'Unable to parse HTML content';
    }
  }
</script>

<div class="watchpoint">
  <h4>{data.watchpoint.name}</h4>
  <ul class="content">
    <li>Status: <StatusIcon status={data.status} />&nbsp;{statusMsg}</li>
    {#if data.watchpoint.kind === 'url' || data.watchpoint.kind === 'keyword'}
      <li>
        URL: <a href={data.watchpoint.target} class="url" target="_blank" rel="noreferrer">{data.watchpoint.target}</a>
      </li>
    {/if}
    {#if data.watchpoint.kind === 'keyword'}
      <li>Keyword: <span class="keyword">{data.watchpoint.keyword}</span></li>
    {/if}
  </ul>
</div>

<style lang="scss">
  .watchpoint {
    position: relative;
    flex: 1 0 0;
    border: 1px solid var(--c-foreground);
    min-width: 300px;

    &:after {
      content: '';
      display: block;
      position: absolute;
      bottom: 0;
      right: 0;
      border: 2px solid var(--c-foreground);
      border-top: 0;
      border-left: 0;
      width: 33.3334%;
      height: 25%;
      transition: all 0.3s;
    }

    &:hover:after {
      width: 50%;
      height: 40%;
    }

    a {
      &,
      &:visited {
        color: var(--c-foreground);
      }

      &:hover,
      &:active {
        background: var(--c-foreground);
        color: var(--c-background);
      }
    }
  }

  h4 {
    background: var(--c-foreground);
    color: var(--c-background);
    letter-spacing: 0.15em;
    margin: 0;
    padding: 8px;
  }

  .content {
    padding: 16px 8px 8px;
    margin: 0;
    list-style: none;
  }

  .url,
  .keyword {
    font-family: monospace;
  }
</style>
