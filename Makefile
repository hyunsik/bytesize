SHELL := /bin/bash
.PHONY: all

all:
	cargo sort -w
	cargo fmt --all
	cargo clippy --all-features
	RUST_BACKTRACE=1 cargo test --all-features
