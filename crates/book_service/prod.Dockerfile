FROM rust:1.96.0-slim AS builder

RUN apt-get update && apt-get install -y \
    build-essential mold && rm -rf /var/lib/apt/lists/*

WORKDIR /build/crates/book_service

ENV CARGO_TARGET_DIR=/build/target
ENV RUSTFLAGS="-C link-arg=-fuse-ld=mold"

RUN mkdir -p /service

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,from=workspace_root,source=Cargo.lock,target=/build/Cargo.lock \
    --mount=type=bind,from=workspace_root,source=Cargo.toml,target=/build/Cargo.toml \
    --mount=type=bind,source=toasty,target=toasty \
    --mount=type=bind,source=Toasty.toml,target=Toasty.toml \
    --mount=type=cache,target=/build/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
cp /build/target/release/app /service/app
cp /build/target/release/migration /service/migration
cp -r ./toasty /service/toasty/
cp ./Toasty.toml /service/Toasty.toml
EOF

# ==============================================================================
FROM gcr.io/distroless/cc-debian13:nonroot
WORKDIR /service

COPY --from=builder /service/app /service/app
COPY --from=builder /service/migration /service/migration
COPY --from=builder /service/Toasty.toml /service/Toasty.toml
COPY --from=builder /service/toasty /service/toasty/

CMD ["/service/app"]