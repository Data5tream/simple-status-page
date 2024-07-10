# Simple Status Page

[![ci](https://github.com/Data5tream/simple-status-page/actions/workflows/ci.yml/badge.svg)](https://github.com/Data5tream/simple-status-page/actions/workflows/ci.yml)
[![Lint backend](https://github.com/Data5tream/simple-status-page/actions/workflows/lint-backend.yml/badge.svg)](https://github.com/Data5tream/simple-status-page/actions/workflows/lint-backend.yml)
[![Lint frontend](https://github.com/Data5tream/simple-status-page/actions/workflows/lint-frontend.yml/badge.svg)](https://github.com/Data5tream/simple-status-page/actions/workflows/lint-frontend.yml)
![GitHub tag (with filter)](https://img.shields.io/github/v/tag/data5tream/simple-status-page)
![GitHub](https://img.shields.io/github/license/data5tream/simple-status-page)


A web app that does status checking. Configured through a simple `config.toml` (or environment variables).

## Usage

Copy the `docker-compose.yml` and `config_example.toml` to a local folder. Rename `config_example.toml` to `config.toml`
and fill out the configuration. Then run:

```shell
docker compose up -d
```

This will pull the latest `redis` and `simple-status-page` images and run them. The status page will be available on port
`8000`.

It's recommended to run the app behind a reverse proxy that does TLS termination, like [caddy](https://caddyserver.com/).

Configuration can also be done via environmental variables. The env vars take precedence over the config file.

## Development

### Environment

The following environment variables need to be set (defaults for a local environment with a running redis instance):

- `APP_REDIS_URL=redis://localhost:6379/0`
- `APP_WEBSERVER_HOST=0.0.0.0`
- `APP_WEBSERVER_PORT=8000`
- `APP_WEBSERVER_URL=http://localhost:5173`

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

## License

Simple Status Page - a simple status web app built with rust

Copyright (C) 2023  Simon Stefan Barth

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
