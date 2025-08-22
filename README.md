# Simple Status Page

[![ci](https://img.shields.io/github/actions/workflow/status/data5tream/simple-status-page/ci.yml?style=for-the-badge)](https://github.com/Data5tream/simple-status-page/actions/workflows/ci.yml)
[![Lint backend](https://img.shields.io/github/actions/workflow/status/data5tream/simple-status-page/lint-backend.yml?style=for-the-badge&label=Backend%20lints)](https://github.com/Data5tream/simple-status-page/actions/workflows/lint-backend.yml)
[![Lint frontend](https://img.shields.io/github/actions/workflow/status/data5tream/simple-status-page/lint-frontend.yml?style=for-the-badge&label=Frontend%20lints)](https://github.com/Data5tream/simple-status-page/actions/workflows/lint-frontend.yml)
![GitHub tag (with filter)](https://img.shields.io/github/v/tag/data5tream/simple-status-page?style=for-the-badge)
![GitHub](https://img.shields.io/github/license/data5tream/simple-status-page?style=for-the-badge&color=blue)


> A web app that does status checking. Configured through a simple `config.toml` (or environment variables).

## Usage

Copy the `compose.yaml` and `config_example.toml` to a local folder. Rename `config_example.toml` to `config.toml`
and fill out the configuration. Then run:

```shell
docker compose up -d
```

This will pull the latest `simple-status-page` image and run it. The status page will be available on port
`8000`.

It's recommended to run the app behind a reverse proxy that does TLS termination.

Configuration can also be done via environmental variables. The env vars take precedence over the config file.

## Development

Copy the example development config filter

```shell
cp config_dev_example.toml config.toml
```

### Backend

[Actix Web](https://actix.rs/) with [sled](https://sled.rs/) as a datastore.

```shell
watchexec -w src -r cargo run
```

### Frontend

[Angular](https://angular.dev/) webapp. Set the `webserver.url` config value to your svelte
`host:port`, otherwise you will have CORS issues.

```shell
pnpm i
pnpm run dev
```

## License

As of v0.9.0, this software is licensed under GPL-v3.0. Prior versions were published under
AGPL.

Simple Status Page - a simple service status app built with rust

Copyright (C) 2023-2025  Simon Stefan Barth

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
