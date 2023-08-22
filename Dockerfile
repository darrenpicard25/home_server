FROM rust:alpine AS environment_builder
WORKDIR /build

RUN apk update && \
    apk upgrade --no-cache && \
    apk add pkgconfig musl-dev npm bash binaryen gcc git g++ libc-dev make openssl-dev protobuf-dev protoc

RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos

FROM environment_builder as builder

COPY . .

RUN cargo leptos build --release


FROM scratch
WORKDIR /app

ENV LEPTOS_OUTPUT_NAME=member_portal
ENV LEPTOS_SITE_ROOT=site
ENV LEPTOS_SITE_PKG_DIR=pkg
ENV LEPTOS_SITE_ADDR="0.0.0.0:3002"
ENV LEPTOS_RELOAD_PORT=3002

USER 10001

COPY --chown=10001:10001 --from=builder /build/target/site/ ./site/
COPY --chown=10001:10001 --from=builder /build/target/server/release/server-package ./server

EXPOSE 3002

CMD ["/app/server"]