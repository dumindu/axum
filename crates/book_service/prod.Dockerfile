FROM rust:1.96.0-slim AS builder
WORKDIR /build/crates/book_service

ENV CARGO_TARGET_DIR=/build/target

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,from=workspace_root,source=Cargo.lock,target=/build/Cargo.lock \
    --mount=type=bind,from=workspace_root,source=Cargo.toml,target=/build/Cargo.toml \
    --mount=type=cache,target=/build/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
cp /build/target/release/book_service /usr/local/bin/book_service
EOF

# ==============================================================================
FROM gcr.io/distroless/cc-debian13:nonroot
WORKDIR /book_service

COPY --from=builder /usr/local/bin/book_service /book_service/app

CMD ["/book_service/app"]