<script lang="ts">
  import { building } from '$app/environment';
  import { env } from '$env/dynamic/public';

  const title = building ? 'Simple Status Page' : env.PUBLIC_PAGE_TITLE ?? 'Simple Status Page';
</script>

<svelte:head>
  <title>{title}</title>
</svelte:head>

<h1>{title}</h1>

<style lang="scss">
  h1 {
    display: inline-block;
    background: var(--c-background);
    color: var(--c-foreground);
    padding: 8px 16px 8px 8px;
    font-family: monospace;

    &:after {
      content: '_';
      animation: blink 2s infinite;
    }
  }
</style>
