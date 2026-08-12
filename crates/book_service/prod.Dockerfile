FROM rust:1.97-slim AS builder

ARG CARGO_BUILD_TARGET=aarch64-unknown-linux-musl

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential musl-tools && rm -rf /var/lib/apt/lists/*

RUN rustup target add ${CARGO_BUILD_TARGET}

WORKDIR /build/crates/book_service

ENV CARGO_TARGET_DIR=/build/target \
    CARGO_BUILD_TARGET=${CARGO_BUILD_TARGET}

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
cp /build/target/${CARGO_BUILD_TARGET}/release/app /service/app
cp /build/target/${CARGO_BUILD_TARGET}/release/migration /service/migration
cp -r ./toasty /service/toasty/
cp ./Toasty.toml /service/Toasty.toml
EOF

# ==============================================================================
FROM gcr.io/distroless/static-debian13:nonroot
WORKDIR /service

COPY --from=builder /service/app /service/app
COPY --from=builder /service/migration /service/migration
COPY --from=builder /service/Toasty.toml /service/Toasty.toml
COPY --from=builder /service/toasty /service/toasty/

CMD ["/service/app"]