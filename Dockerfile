FROM node:24.0 AS frontend-base

ENV APP_WEBSERVER_HOST="0.0.0.0" APP_WEBSERVER_PORT="8000" APP_WEBSERVER_URL="http://localhost:8000" \
    APP_LOG_LEVEL="info" APP_LOG_PATH="/var/log/status-page.log" NODE_ENV="production" PUBLIC_API_URL=""
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

COPY frontend /frontend
WORKDIR /frontend

FROM frontend-base AS frontend-prod-deps
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --prod --frozen-lockfile

FROM frontend-base AS frontend
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm run build


# Build healthcheck
FROM rust:1.88-bookworm AS healthcheck-builder

RUN cargo install simple-web-healthcheck

# Build app
FROM rust:1.88 AS backend
WORKDIR /usr/src/status-page
COPY ["Cargo.lock", "Cargo.toml", "./"]
COPY src src/
RUN cargo install --path .

# Build final image
FROM gcr.io/distroless/cc-debian12

ENV APP_WEBSERVER_HOST="0.0.0.0" APP_WEBSERVER_PORT="8000"

COPY --from=healthcheck-builder /usr/local/cargo/bin/simple-web-healthcheck /healthcheck
COPY --from=backend /usr/local/cargo/bin/simple-status-page app
COPY --from=frontend /frontend/dist/frontend/browser web

EXPOSE 8000
HEALTHCHECK --interval=10s --timeout=1s CMD ["/healthcheck", "http://127.0.0.1:8000"]

CMD ["./app"]
