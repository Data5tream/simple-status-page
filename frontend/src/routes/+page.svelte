<script lang="ts">
  import type { StatusData } from '$lib/dataprovider';

  import '$lib/main.css';

  import PageFooter from '$lib/components/PageFooter.svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import StatusList from '$lib/components/StatusList.svelte';
  import StatusOverview from '$lib/components/StatusOverview.svelte';

  export let data: { status: StatusData };
</script>

<main>
  <PageHeader />
  {#if data.status && !data.status.is_valid}
    <span class="error">Error loading status data :(</span>
  {:else}
    <StatusOverview bind:data={data.status.watchpoints} />
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

  .error {
    display: block;
    color: var(--c-red);
    font-size: 64px;
  }
</style>
