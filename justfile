build *ARGS:
    cargo build {{ARGS}}

fmt:
    cargo fmt

serve *ARGS:
    cargo run --package entrypoint {{ARGS}}

transpile *ARGS: fmt
    cargo run --package cranz {{ARGS}}
