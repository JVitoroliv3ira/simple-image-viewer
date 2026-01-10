BIN := siv
ARGS ?=

.PHONY: help dev run build release fmt clippy test clean

help:
	@echo "Targets:"
	@echo "  make dev                 # cargo run (debug) com backtrace"
	@echo "  make run ARGS='img.png'  # roda passando argumentos"
	@echo "  make build               # compila debug"
	@echo "  make release             # compila release"
	@echo "  make fmt                 # rustfmt"
	@echo "  make clippy              # clippy (warnings como erro)"
	@echo "  make test                # tests"
	@echo "  make clean               # limpa target/"

dev:
	@RUST_BACKTRACE=1 cargo run -- $(ARGS)

run:
	@cargo run -- $(ARGS)

build:
	@cargo build

release:
	@cargo build --release

fmt:
	@cargo fmt

clippy:
	@cargo clippy -- -D warnings

test:
	@cargo test

clean:
	@cargo clean

