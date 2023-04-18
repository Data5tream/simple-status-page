# Simple Status Page

A web app that does status checking. Configured through a simple `config.toml` (or environment variables).

## Usage

Copy the `docker-compose.yml` and `config_example.toml` to a local folder. Rename `config_example.toml` to `config.toml`
and fill out the configuration. Then run:

```shell
docker compose up -d
```

This will pull the latest `redis` and `simple-status-page` images and run them. The status page will be available on port
`8080`.

It's recommended to run the app behind a reverse proxy that does TLS termination, like [caddy](https://caddyserver.com/).

Configuration can also be done via environmental variables. The env vars take precedence over the config file.

## Development

### Backend

[Actix Web](https://actix.rs/) with [Redis](https://redis.io/) as a datastore. You will need a locally available redis
instance for development.

```shell
cargo run
```

### Frontend

[SvelteKit](https://kit.svelte.dev/) Webapp written in TypeScript. Set the `webserver.url` config value to your svelte
`host:port`, otherwise you will have CORS issues.

```shell
pnpm i
pnpm run dev
```
