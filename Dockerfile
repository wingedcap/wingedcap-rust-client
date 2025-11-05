FROM rust:1.90-bookworm AS builder

ARG PORT
ARG HUB_HOST
ARG HUB_PK

WORKDIR /app

COPY . .

RUN cargo install cargo-make

RUN cargo make setup

RUN apt update
RUN apt-get install -y pkg-config libssl-dev

RUN cargo install cargo-binstall

RUN cargo binstall dioxus-cli@0.6.3

RUN cargo make build-web

FROM nginx:alpine

COPY --from=builder /app/target/dx/wingedcap-client/release/web/public /usr/share/nginx/html

CMD ["nginx", "-g", "daemon off;"]
