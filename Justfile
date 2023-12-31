#!/usr/bin/env -S just --justfile

set dotenv-load := true

profile := env_var_or_default("PROFILE", "dev")
features := env_var_or_default("FEATURES", "")
feature_args := if features == "" { "" } else { "--no-default-features --features " + features } 

default:
    @just --list

clean:
    cargo clean

# Build library
build:
    cargo build --profile={{ profile }} --all-targets {{ feature_args }}

test:
    cargo test --profile={{ profile }} --all-targets {{ feature_args }}

lint:
    cargo clippy --profile={{ profile }} --all-targets {{ feature_args }} -- -D warnings

# Check for known vulnerabilities in dependencies
audit:
    cargo audit

# TODO reinstate audit
#check: && lint test audit
# cargo fmt --check --all

check: && lint test
    cargo +nightly fmt --check --all

format:
    cargo +nightly fmt --all

update_deps:
    cargo update

unused_deps:
    cargo +nightly udeps --all-targets




# Install tooling
setup env="dev":
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh {{ if env == "ci" { "-s -- -y" } else { "" } }}
    rustup install nightly
    rustup show
    cargo install cargo-audit
