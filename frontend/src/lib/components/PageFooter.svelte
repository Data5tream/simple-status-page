<footer>
  <div class="content">
    &copy; {new Date().getFullYear()}, Simon Barth |
    <a href="https://github.com/Data5tream/simple-status-page" target="_blank" rel="noreferrer">GitHub</a>
  </div>
</footer>

<style lang="scss">
  footer {
    width: 100%;
    padding: 12px 0 8px;
    background: var(--c-foreground);
    color: var(--c-background);
  }

  .content {
    max-width: 1400px;
    margin: 0 16px;
    font-family: monospace;
    font-size: 14px;

    @media screen and (min-width: 1432px) {
      width: 100%;
      margin: 0 auto;
    }
  }

  a {
    &,
    &:visited {
      color: var(--c-background);
    }

    &:hover,
    &:active {
      background: var(--c-background);
      color: var(--c-foreground);
    }
  }
</style>
