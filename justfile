_default:
  just --list -u

build:
    cargo build --workspace --all-targets --verbose

fmt:
    cargo fmt --all
    dprint fmt "**/*"

fmt-check:
    cargo fmt --all --check
    dprint check "**/*"

lint:
    cargo clippy --workspace --all-targets -- -D warnings -D clippy::pedantic

lint-fix:
    cargo clippy --fix --allow-dirty --workspace --all-targets -- -D warnings -D clippy::pedantic

test-node:
    pnpm exec vitest run

test-rust:
    cargo test --workspace --all-targets --verbose

test-all:
    just test-rust
    just test-node
