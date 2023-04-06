# Simple Status Page

A simple web app that does status checking. Configured through a simple `config.toml`.

## Structure

### Backend

[Actix Web](https://actix.rs/) with [Redis](https://redis.io/) as a datastore.

```shell
cargo run
```

### Frontend

[SvelteKit](https://kit.svelte.dev/) Webapp written in TypeScript.

```shell
pnpm i
pnpm run dev
```
