clean *ARGS:
	cargo clean {{ARGS}}

serve *ARGS:
	cargo watch -xrun {{ARGS}}

test *ARGS: test_clean
	hurl --very-verbose --report-html report --test test.hurl {{ARGS}}

test_clean *ARGS:
	hurl --very-verbose clean.hurl {{ARGS}} || true
