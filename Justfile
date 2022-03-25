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

build-example-plugin PLUGIN:
    cargo build --locked --package {{PLUGIN}} --target wasm32-unknown-unknown

run-example PATH WASM:
    cargo run --locked --package {{PATH}} -- {{WASM}}

docs:
    cargo doc --locked --package wasm-plugins --all-features

test-all:
    cargo test --locked
    cargo test --locked --package wasm-plugins --features wasmer-rt-sys
    cargo test --locked --package wasm-plugins --features wasmer

nextest-all:
    cargo nextest run
    cargo nextest run --package wasm-plugins --features wasmer-rt-sys
    cargo nextest run --package wasm-plugins --features wasmer

clean:
    cargo clean --locked
