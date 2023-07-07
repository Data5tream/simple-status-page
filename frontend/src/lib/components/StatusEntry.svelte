<script lang="ts">
  import type { Watchpoint } from '../dataprovider';
  import StatusIcon from './StatusIcon.svelte';

  export let data: Watchpoint;

  let statusMsg = '';
  $: if (data) {
    if (data.status < 600) {
      statusMsg = `${data.status}`;
    } else if (data.status === 600) {
      statusMsg = 'Invalid DNS';
    } else if (data.status === 601) {
      statusMsg = 'TLS validation error';
    }
  }
</script>

<div class="watchpoint">
  <h4>{data.watchpoint.name}</h4>
  <div class="content">
    Status: <StatusIcon status={data.status} />&nbsp;{statusMsg}<br />
    URL: <a href={data.watchpoint.url} class="url">{data.watchpoint.url}</a>
  </div>
</div>

<style lang="scss">
  .watchpoint {
    position: relative;
    flex: 1 0 0;
    border: 1px solid var(--c-black);
    min-width: 300px;

    &:after {
      content: '';
      display: block;
      position: absolute;
      bottom: 0;
      right: 0;
      border: 2px solid var(--c-black);
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
        color: var(--c-black);
      }

      &:hover,
      &:active {
        background: var(--c-black);
        color: var(--c-white);
      }
    }
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
