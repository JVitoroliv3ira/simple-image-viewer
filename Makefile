BIN := siv
ARGS ?=

.PHONY: help dev run build release fmt clippy test clean build-run run-release

help:
	@echo "Targets:"
	@echo "  make dev                      # cargo run (debug) com backtrace"
	@echo "  make run ARGS='img.png'       # cargo run passando argumentos"
	@echo "  make build                    # compila debug"
	@echo "  make release                  # compila release"
	@echo "  make build-run ARGS='img.png' # build (debug) + roda binario gerado"
	@echo "  make run-release ARGS='img.png' # build (release) + roda binario gerado"
	@echo "  make fmt                      # rustfmt"
	@echo "  make clippy                   # clippy (warnings como erro)"
	@echo "  make test                     # tests"
	@echo "  make clean                    # limpa target/"

dev:
	@RUST_BACKTRACE=1 cargo run -- $(ARGS)

run:
	@cargo run -- $(ARGS)

build:
	@cargo build

release:
	@cargo build --release

# ---- novos: roda o binario gerado ----
build-run: build
	@./target/debug/$(BIN) $(ARGS)

run-release: release
	@./target/release/$(BIN) $(ARGS)
# --------------------------------------

fmt:
	@cargo fmt

clippy:
	@cargo clippy -- -D warnings

test:
	@cargo test

clean:
	@cargo clean
