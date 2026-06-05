# Run lints on the workspace members (cargo fmt and clippy)
lint:
    cargo +nightly fmt --all --check
    cargo clippy --workspace --all-targets -- -D warnings

# Run cargo check on the workspace members
check:
    cargo check --workspace

# Run cargo build on the workspace members
build:
    cargo build --workspace --all-targets

# Run cargo clean on the workspace members
clean:
    cargo clean

# Run cargo test on the workspace members
test:
    cargo test --workspace

# Forward to the BOOK-SERVICE
mod book "crates/book_service"