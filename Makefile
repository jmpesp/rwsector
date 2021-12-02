
.PHONY: all clippy fmt test build clean fix doc alpine illumos

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

alpine:
	rustup target add x86_64-unknown-linux-musl
	cargo build --release --target x86_64-unknown-linux-musl

illumos:
	rustup target add x86_64-unknown-illumos
	cargo build --release --target x86_64-unknown-illumos
