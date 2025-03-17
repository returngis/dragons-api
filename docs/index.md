# About this project

This example is a REST API that manages dragons 🐉. It uses Rust as a language.

## What is Rust?

Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. It is a language that is focused on performance, reliability, and productivity. Rust is a great language for building reliable and efficient software.

## Dependencies

- [warp](https://crates.io/crates/warp) - A web server framework for Rust
- [serde](https://crates.io/crates/serde) - A framework for serializing and deserializing Rust data structures efficiently and generically.
- [chrono](https://crates.io/crates/chrono) - Has utilities for parsing and formatting dates and times.
- [tokio](https://crates.io/crates/tokio) - A library for writing reliable asynchronous applications with Rust.
- [pretty_env_logger](https://crates.io/crates/pretty_env_logger) - A logger that logs to stdout and is configurable via environment variables.
- [uuid](https://crates.io/crates/uuid) - A library for creating, parsing, and working with UUIDs.

You can check those dependencies in the `Cargo.toml` file.

## How to run this project

Install `cargo-watch`:

```bash
cargo install cargo-watch
```

Run the project:

```bash
cargo watch -q -c -w src/ -x run
```

This command will watch the `src/` directory and run the project every time a file changes. The `-q` flag will suppress the output of the `cargo-watch` command, and the `-c` flag will clear the screen before running the project. The `-w` flag specifies the directory to watch, and the `-x` flag specifies the command to run.
