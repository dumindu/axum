FROM rust:1.96.0-slim
WORKDIR /build/crates/book_service

ENV CARGO_TARGET_DIR=/build/target

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

WORKDIR /service
CMD ["/service/app"]

EXPOSE 3000