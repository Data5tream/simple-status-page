FROM node:20.5 AS frontend

WORKDIR /frontend
RUN npm install -g pnpm
COPY frontend .
RUN pnpm i
ENV NODE_ENV=production
ENV PUBLIC_API_URL=""
RUN pnpm run build

FROM rust:1.71 AS backend
WORKDIR /usr/src/status-page
COPY ["Cargo.lock", "Cargo.toml", "./"]
RUN cargo
COPY src src/
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=backend /usr/local/cargo/bin/simple-status-page app
COPY --from=frontend /frontend/build web

CMD ["./app"]
