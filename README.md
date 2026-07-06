[![buymeacoffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-dumindu-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=ffffff&labelColor=333333)](https://www.buymeacoffee.com/dumindu)

[![learning-rust.github.io](https://img.shields.io/github/stars/learning-rust/learning-rust.github.io?style=for-the-badge&logo=rust&label=learning-rust.github.io&labelColor=333333&color=F46623)](https://learning-rust.github.io)
[![learning-cloud-native-go/myapp](https://img.shields.io/github/stars/learning-cloud-native-go/myapp?style=for-the-badge&logo=go&logoColor=ffffff&label=learning-cloud-native-go%2Fmyapp&labelColor=333333&color=00ADD8)](https://github.com/learning-cloud-native-go/myapp)
[![learning-cloud-native-go.github.io](https://img.shields.io/github/stars/learning-cloud-native-go/learning-cloud-native-go.github.io?style=for-the-badge&logo=go&logoColor=ffffff&label=learning-cloud-native-go.github.io&labelColor=333333&color=00ADD8)](https://learning-cloud-native-go.github.io)
[![E25DX](https://img.shields.io/github/stars/dumindu/E25DX?style=for-the-badge&logo=hugo&logoColor=ffffff&label=E25DX&labelColor=333333&color=FF4088)](https://themes.gohugo.io/themes/e25dx/)

# Axum with Toasty ORM

## Docker images

- dev: rust:1.96.0 slim
- prod: distroless cc-debian13:nonroot

| Environment    | Rust Image Size | Postgres Image Size |
|----------------|-----------------|---------------------|
| Development    | 900 MB          | 300MB               |
| Production     | 70 MB           |                     |

## Just commands

```just
$~/dev/learning-rust/axum  just
🚀AXUM
  help     # List available commands
  lint     # Run lints on the workspace members (cargo fmt and clippy)
  check    # Run cargo check on the workspace members
  build    # Run cargo build on the workspace members
  clean    # Run cargo clean on the workspace members
  test     # Run cargo test on the workspace members
  book ... # Forward to the BOOK-SERVICE

  📖BOOK SERVICE
    help           # List available commands
    lint           # Run lints (cargo fmt and clippy)
    check          # Run cargo check
    build          # Run cargo build
    clean          # Run cargo clean on the book_service package
    test           # Run cargo test
    migration *cmd # Run DB migrate
    app            # Run server app
    docker *cmd    # Run docker commands
    build-for-prod # Build production distroless image
```

## Project structure

```shell
.
├── Cargo.toml
├── Cargo.lock
├── crates
│   ├── book_service
│   │   ├── justfile
│   │   ├── openapi.yaml
│   │   ├── compose.yml
│   │   ├── Dockerfile
│   │   ├── prod.Dockerfile
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   ├── lib.rs
│   │   │   ├── state.rs
│   │   │   ├── routes.rs
│   │   │   ├── config.rs
│   │   │   ├── errors.rs
│   │   │   ├── openapi.rs
│   │   │   ├── bin
│   │   │   │   ├── app.rs
│   │   │   │   ├── migration.rs
│   │   │   │   └── apidoc.rs
│   │   │   ├── app
│   │   │   │   ├── mod.rs
│   │   │   │   ├── book
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── handler.rs
│   │   │   │   │   └── payload.rs
│   │   │   │   └── shared
│   │   │   │       ├── mod.rs
│   │   │   │       ├── pagination.rs
│   │   │   │       └── validation.rs
│   │   │   └── models
│   │   │       ├── mod.rs
│   │   │       ├── author.rs
│   │   │       └── book.rs
│   │   ├── toasty
│   │   │   ├── history.toml
│   │   │   ├── migrations
│   │   │   │   └── 0000_migration.sql
│   │   │   └── snapshots
│   │   │       └── 0000_snapshot.toml
│   │   └── Toasty.toml
│   └── README.md
├── justfile
├── rustfmt.toml
└── LICENSE
```
