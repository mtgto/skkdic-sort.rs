ARG RUST_VERSION=1.82.0
FROM rust:${RUST_VERSION}-bullseye AS builder

WORKDIR /app
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    cp ./target/release/skkdic-sort /bin/skkdic-sort

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /bin/skkdic-sort /usr/bin/skkdic-sort
ENTRYPOINT ["/usr/bin/skkdic-sort"]
