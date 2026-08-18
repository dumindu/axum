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

## Just commands

```justfile
$just
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

## Database Design

To keep this simple, we use only a single database table named `books`.

| Column Name    | Datatype    | Not Null | Primary Key |
|----------------|-------------|----------|-------------|
| created_at     | TIMESTAMPTZ | ✅       |             |
| updated_at     | TIMESTAMPTZ | ✅       |             |
| id             | UUID        | ✅       | ✅          |
| published_date | DATE        | ✅       |             |
| status         | SMALLINT    | ✅       |             |
| title          | TEXT        | ✅       |             |
| description    | TEXT        |          |             |
| image_url      | TEXT        |          |             |

> [!important]
> - For high-traffic systems with very large PostgreSQL tables that containing millions/billions of rows, arranging fixed-width columns by decreasing alignment requirements can reduce tuple alignment padding; potentially minimize row/ storage size. This technique is called "**Column Tetris**".
> - For this optimization, order fixed-width table columns by decreasing alignment requirements.
>   - 8-byte alignment types: `bigint`, `bigserial`, `double precision`/ `float8`, `timestamp`, `timestamptz`, `time`, `interval`
>   - 4-byte alignment types: `integer`, `serial`, `real`/ `float4`, `uuid`, `date`
>   - 2-byte alignment types: `smallint`, `smallserial`
>   - 1-byte alignment types: `boolean`
>   - Variable-width types (at last): `numeric`, `text`, `character varying`/ `varchar`, `bytea`
> - However, it's ok to follow a more readable column format, when your table schema changes frequently.

## Endpoints

| Name        | HTTP Method | Route          |
|-------------|-------------|----------------|
| List Books  | GET         | /v1/books      |
| Create Book | POST        | /v1/books      |
| Read Book   | GET         | /v1/books/{id} |
| Update Book | PUT         | /v1/books/{id} |
| Delete Book | DELETE      | /v1/books/{id} |
| Health      | GET         | /livez         |

### Request (`POST`/`PUT`)

```json
{
  "title": "Harry Potter and the Deathly Hallows",
  "description": "It is the seventh and final novel in the Harry Potter series",
  "image_url": "https://upload.wikimedia.org/wikipedia/en/a/a9/Harry_Potter_and_the_Deathly_Hallows.jpg",
  "published_date": "2007-07-21",
  "status": "verified"
}
```

### Response (`GET`/`POST`/`PUT`)
```json
{
  "created_at": "2027-01-01T00:00:00.123456Z",
  "updated_at": "2027-01-01T00:00:00.123456Z",
  "id": "01bbbbbb-bbbb-7bbb-8bbb-bbbbbbbbbbbb",
  "published_date": "2007-07-21",
  "status": "verified",
  "title": "Harry Potter and the Deathly Hallows",
  "description": "It is the seventh and final novel in the Harry Potter series",
  "image_url": "https://upload.wikimedia.org/wikipedia/en/a/a9/Harry_Potter_and_the_Deathly_Hallows.jpg"
}
```

> [!note]
> The list endpoint returns an array of above response JSON.

## Form Validation

```json
{
  "errors": {
    "title": "Must be at least 1 character long",
    "image_url": "Must be a valid URL"
  }
}
```

## Project Structure

```shell
rest_api_workspace
├── crates
│   ├── book_service
│   │   ├── src
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
│   │   │   ├── models
│   │   │   │   ├── mod.rs
│   │   │   │   └── book.rs
│   │   │   ├── config.rs
│   │   │   ├── errors.rs
│   │   │   ├── state.rs
│   │   │   ├── routes.rs
│   │   │   ├── openapi.rs
│   │   │   └── lib.rs
│   │   ├── Toasty.toml
│   │   ├── toasty
│   │   │   ├── history.toml
│   │   │   ├── migrations
│   │   │   │   └── 0000_migration.sql
│   │   │   └── snapshots
│   │   │       └── 0000_snapshot.toml
│   │   ├── Cargo.toml
│   │   ├── openapi.yaml
│   │   ├── compose.yml
│   │   ├── prod.Dockerfile
│   │   ├── Dockerfile
│   │   └── justfile
│   └── README.md
├── Cargo.lock
├── Cargo.toml
├── rustfmt.toml
├── LICENSE
├── README.md
└── justfile
```

## Containerization Environment

| Environment    | Rust Image Type                    | Rust Image Size | Postgres Image Type | Postgres Image Size |
|----------------|------------------------------------|-----------------|---------------------|---------------------|
| Development    | rust:1.97-slim                     | ~ 900 MB        | postgres:18-alpine  | ~ 300MB             |
| Production     | distroless/static-debian13:nonroot | ~ 15 MB         |                     |                     |
