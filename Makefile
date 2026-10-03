.PHONY: build test fmt lint clean

build:
	stellar contract build

test:
	cargo test

fmt:
	cargo fmt --all

lint:
	cargo clippy --all-targets -- -D warnings

clean:
	cargo clean
