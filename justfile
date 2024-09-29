build *ARGS:
	cargo build {{ARGS}}

check *ARGS:
	cargo check {{ARGS}}

clean *ARGS:
	cargo clean {{ARGS}}

format *ARGS:
	cargo fmt {{ARGS}}

lint *ARGS:
	cargo clippy {{ARGS}}

run *ARGS:
	cargo run {{ARGS}}

sanity *ARGS: sanity_clean
	hurl --very-verbose --report-html report --test test.hurl {{ARGS}}

sanity_clean *ARGS:
	hurl --very-verbose clean.hurl {{ARGS}} || true

serve *ARGS:
	cargo watch -x"run --package entrypoint" {{ARGS}}

test *ARGS:
	cargo test {{ARGS}}

transpile *ARGS:
	cargo run --package cranz {{ARGS}}

watch *ARGS:
	cargo watch {{ARGS}}
