
.PHONY: all clippy fmt test build clean fix doc

all: clippy

clippy: fmt
	cargo clippy

fmt: test
	cargo fmt

test: build
	cargo test

build:
	cargo build

clean:
	cargo clean

fix:
	cargo fix --allow-dirty

doc:
	cargo readme > README.md

alpine:
	rustup target add x86_64-unknown-linux-musl
	cargo build --target x86_64-unknown-linux-musl
