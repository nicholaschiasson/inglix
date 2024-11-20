build *ARGS:
	cargo build {{ARGS}}

check *ARGS:
	cargo check {{ARGS}}

clean *ARGS:
	rm -rf report
	cargo clean {{ARGS}}

format *ARGS:
	nixfmt $(find . -type f -name "*.nix")
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
	cargo watch -i report -x"run" {{ARGS}}

test *ARGS:
	cargo test {{ARGS}}

transpile *ARGS:
	cargo run --package cranz {{ARGS}}

watch *ARGS:
	cargo watch {{ARGS}}
