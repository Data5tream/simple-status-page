FROM node:23.7 AS frontend

ENV APP_WEBSERVER_HOST="0.0.0.0" APP_WEBSERVER_PORT="8000" APP_WEBSERVER_URL="http://localhost:8000" \
    APP_LOG_LEVEL="info" APP_LOG_PATH="/var/log/status-page.log"

WORKDIR /frontend
RUN npm install -g pnpm
COPY frontend .
RUN pnpm i
ENV NODE_ENV=production PUBLIC_API_URL=""
RUN pnpm run build


# Build healthcheck
FROM rust:1.87-bookworm AS healthcheck-builder

RUN cargo install simple-web-healthcheck

# Build app
FROM rust:1.87 AS backend
WORKDIR /usr/src/status-page
COPY ["Cargo.lock", "Cargo.toml", "./"]
RUN cargo
COPY src src/
RUN cargo install --path .

# Build final image
FROM gcr.io/distroless/cc-debian12

ENV APP_WEBSERVER_HOST="0.0.0.0" APP_WEBSERVER_PORT="8000"

COPY --from=healthcheck-builder /usr/local/cargo/bin/simple-web-healthcheck /healthcheck
COPY --from=backend /usr/local/cargo/bin/simple-status-page app
COPY --from=frontend /frontend/build web

EXPOSE 8000
HEALTHCHECK --interval=10s --timeout=1s CMD ["/healthcheck", "http://127.0.0.1:8000"]

CMD ["./app"]
