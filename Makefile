.PHONY: help setup fmt check test build release run clean ci install uninstall

ARGS ?=
INSTALL_ARGS ?=
UNINSTALL_ARGS ?=
POWERSHELL ?= powershell

help:
	@echo "Forge project tasks:"
	@echo "  make setup       Fetch dependencies"
	@echo "  make fmt         Check Rust formatting"
	@echo "  make check       Run cargo check"
	@echo "  make test        Run all tests"
	@echo "  make build       Build debug binary"
	@echo "  make release     Build release binary"
	@echo "  make run ARGS=.. Run Forge with arguments"
	@echo "  make clean       Remove Cargo build output"
	@echo "  make ci          Run formatting, check, tests, and release build"
	@echo "  make install     Run the Windows installer"
	@echo "  make uninstall   Run the Windows uninstaller"

setup:
	cargo fetch

fmt:
	cargo fmt --all -- --check

check:
	cargo check --all-targets --all-features

test:
	cargo test --all-targets --all-features

build:
	cargo build

release:
	cargo build --release

run:
	cargo run -- $(ARGS)

clean:
	cargo clean

ci: fmt check test release

install:
	$(POWERSHELL) -NoProfile -ExecutionPolicy Bypass -File scripts/install.ps1 $(INSTALL_ARGS)

uninstall:
	$(POWERSHELL) -NoProfile -ExecutionPolicy Bypass -File scripts/uninstall.ps1 $(UNINSTALL_ARGS)
