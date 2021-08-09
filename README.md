# Cryptofolio

Rust port of [https://github.com/aeyoll/cryptofolio](cryptofolio).

Requirements
---

- [https://crates.io/crates/sqlx-cli](sqlx-cli): Database management
- [https://github.com/watchexec/cargo-watch](cargo-watch): Server auto-reload for development purpose

Install
---

Database creation:

```sh
sqlx database create
sqlx migrate run
```

Running
---

Dev server:

```sh
cargo watch -x 'run --bin cryptofolio-server'
```