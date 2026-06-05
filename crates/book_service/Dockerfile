FROM rust:alpine
WORKDIR /book_service

RUN apk add --no-cache musl-dev

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
export CARGO_TARGET_DIR=/app/target
cargo build --locked --release
cp /app/target/release/book_service /book_service/main
EOF

CMD ["/book_service/main"]