# SPDX-FileCopyrightText: 2026 Meowdia Community
# SPDX-License-Identifier: MIT OR Apache-2.0

_default:
    just --list

build:
    cargo build --workspace

test:
    cargo test --workspace

iana-fetch:
    cargo run -p xtask -- iana fetch

iana-generate:
    cargo run -p xtask -- iana generate

iana-check:
    cargo run -p xtask -- iana check

iana-update:
    cargo run -p xtask -- iana update

lint:
    cargo clippy --workspace --all-targets -- --deny warnings
    cargo fmt --all --check

reuse:
    reuse lint

check: iana-check lint build test reuse

clean:
    cargo clean

fmt:
    cargo fmt --all
