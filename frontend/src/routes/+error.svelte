<script lang="ts">
  import '$lib/main.css';
  import PageFooter from "$lib/components/PageFooter.svelte";
  import PageHeader from "$lib/components/PageHeader.svelte";
</script>

<main>
    <PageHeader />
        <span class="error">Error loading status data :(</span>
</main>
<PageFooter />

<style lang="scss">
  .error {
    display: block;
    color: var(--c-red);
    font-size: 64px;
  }
</style>