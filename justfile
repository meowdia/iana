# SPDX-FileCopyrightText: 2026 Meowdia Community
# SPDX-License-Identifier: MIT OR Apache-2.0

_default:
    just --list

build:
    cargo build --workspace

test:
    cargo test --workspace
    cargo test -p iana --no-default-features --features metadata
    cargo test -p iana --no-default-features --features sdp-parameters,metadata
    cargo test -p iana --no-default-features --features tls-parameters
    cargo test --workspace --all-features

iana-discover:
    cargo run -p xtask -- iana discover

iana-fetch:
    cargo run -p xtask -- iana fetch

iana-generate:
    cargo run -p xtask -- iana generate

iana-check:
    cargo run -p xtask -- iana check

iana-update:
    cargo run -p xtask -- iana update

lint:
    cargo clippy --workspace --all-targets --all-features -- --deny warnings
    cargo fmt --all --check

reuse:
    reuse lint

check: iana-check lint build test reuse

clean:
    cargo clean

fmt:
    cargo fmt --all
