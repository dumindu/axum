[![buymeacoffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-dumindu-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=ffffff&labelColor=333333)](https://www.buymeacoffee.com/dumindu)

[![learning-rust.github.io](https://img.shields.io/github/stars/learning-rust/learning-rust.github.io?style=for-the-badge&logo=rust&label=learning-rust.github.io&labelColor=333333&color=F46623)](https://learning-rust.github.io)
[![learning-cloud-native-go/myapp](https://img.shields.io/github/stars/learning-cloud-native-go/myapp?style=for-the-badge&logo=go&logoColor=ffffff&label=learning-cloud-native-go%2Fmyapp&labelColor=333333&color=00ADD8)](https://github.com/learning-cloud-native-go/myapp)
[![E25DX](https://img.shields.io/github/stars/dumindu/E25DX?style=for-the-badge&logo=hugo&logoColor=ffffff&label=E25DX&labelColor=333333&color=FF4088)](https://themes.gohugo.io/themes/e25dx/)

# Axum with Toasty ORM

## Batteries Included

- A Rust workspace with Just commands; to lint, build, clean, test, and run each crate.
- Rust linters, Docker, Docker Compose, Alpine development images, and Distroless production images.
- [Axum](https://github.com/tokio-rs/axum) async web framework skeleton with environment-based configurations.
- Production-ready middleware including CORS, Timeout, Structured JSON logging, and Request ID tracking via [Tower](https://github.com/tower-rs/tower) components.
- [Toasty ORM](https://github.com/tokio-rs/toasty) with PostgreSQL support to manage database migrations and type-safe queries.
- [Garde](https://github.com/jprochazk/garde) to validate forms and requests.
- [Utoipa](https://github.com/juhaku/utoipa) to generate OpenAPI v3 specifications.
- [Serde](https://github.com/serde-rs/serde) to serialize and deserialize requests.
- Modern time and date handling using [Jiff](https://github.com/BurntSushi/jiff) with full timezone and Serde integration.
- Cryptographically secure and fast [UUIDv7](https://github.com/uuid-rs/uuid) generation for database primary keys.

### Form Validation

```json
{
  "errors": {
    "title": "Must be at least 1 character long",
    "image_url": "Must be a valid URL"
  }
}
```

## Docker images

- dev: Rust 1.97-slim and Postgres 18-alpine
- prod: Distroless/static-debian13:nonroot

| Environment    | Rust Image Size | Postgres Image Size |
|----------------|-----------------|---------------------|
| Development    | ~ 900 MB        | ~ 300MB             |
| Production     | ~ 30 MB         |                     |

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
    apidoc         # Generate openapi.yaml
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
