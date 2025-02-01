FROM node:23.7 AS frontend

ENV APP_WEBSERVER_HOST="0.0.0.0" APP_WEBSERVER_PORT="8000" APP_WEBSERVER_URL="http://localhost:8000" \
    APP_LOG_LEVEL="info" APP_LOG_PATH="/var/log/status-page.log"

WORKDIR /frontend
RUN npm install -g pnpm
COPY frontend .
RUN pnpm i
ENV NODE_ENV=production PUBLIC_API_URL=""
RUN pnpm run build

FROM rust:1.84 AS backend
WORKDIR /usr/src/status-page
COPY ["Cargo.lock", "Cargo.toml", "./"]
RUN cargo
COPY src src/
RUN cargo install --path .

FROM gcr.io/distroless/cc-debian12
COPY --from=backend /usr/local/cargo/bin/simple-status-page app
COPY --from=frontend /frontend/build web

EXPOSE 8000

CMD ["./app"]
