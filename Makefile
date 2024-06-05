format:
	cargo fmt --all --
.PHONY: format

format-check:
	cargo fmt --all -- --check
.PHONY: format-check

test:
	cargo test --all --all-features --tests
.PHONY: test

clippy:
	cargo clippy --all --all-features --tests -- -D warnings
.PHONY: clippy

doc:
	cargo doc --all-features
.PHONY: doc

all: format clippy test doc