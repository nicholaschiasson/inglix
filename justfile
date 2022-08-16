set shell := ["nu", "-c"]

fmt:
    cargo fmt

transpile *ARGS: fmt
    cargo run --package cranz {{ARGS}}
