# just manual: https://github.com/casey/just/#readme

_default:
    @just --list

check-all:
    cargo clippy --locked -- -D warnings

check-lib:
    cargo clippy --locked --package wasm-plugins -- -D warnings

build-lib:
    cargo build --locked --package wasm-plugins

build-lib-release:
    cargo build --locked --package wasm-plugins --release

build-examples:
    cargo build --locked --example simple-runner
    cargo build --locked --example simple-plugin --target wasm32-unknown-unknown

build-examples-release:
    cargo build --locked --example simple-runner --release
    cargo build --locked --example simple-plugin --target wasm32-unknown-unknown --release

docs:
    cargo doc --locked --package wasm-plugins --all-features

test-all:
    cargo test --locked

clean:
    cargo clean --locked
