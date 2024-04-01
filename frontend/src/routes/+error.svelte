<script lang="ts">
  import '$lib/main.css';
  import PageFooter from '$lib/components/PageFooter.svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
</script>

<main>
  <PageHeader />
  <span class="error">404</span>
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
