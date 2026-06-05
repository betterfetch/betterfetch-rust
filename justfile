#!/usr/bin/env -S just --justfile

# Run the `help` recipe
[group('General')]
default: help

# List available recipes
[group('General')]
help:
    @just --list

# Build the project in release mode
[group('Build')]
build:
    cargo build --release

# Build the project in development mode
[group('Build')]
build-dev:
    cargo build

# Run the project from source
[group('General')]
run *ARGS:
    cargo run {{ ARGS }}

# Install the binary globally
[group('Install')]
install:
    cargo install --path .

# Uninstall the globally installed binary
[group('Install')]
uninstall:
    cargo uninstall betterfetch

# Reinstall the binary
[group('Install')]
reinstall: uninstall install

# Lint with Clippy
[group('Check')]
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Format the code
[group('Check')]
fmt:
    cargo fmt --all

# Check formatting without modifying files
[group('Check')]
fmt-check:
    cargo fmt --all -- --check

# Run tests
[group('Test')]
test:
    cargo test

# Run tests with output
[group('Test')]
test-verbose:
    cargo test -- --nocapture

# Clean build artifacts
[group('Build')]
clean:
    cargo clean

# Clean and rebuild in release mode
[group('Build')]
rebuild: clean build

# Generate and open documentation
[group('Documentation')]
doc:
    cargo doc --open

# Generate documentation without opening
[group('Documentation')]
doc-no-open:
    cargo doc

# Check the project for errors without building
[group('Check')]
check:
    cargo check

# Check all targets and features
[group('Check')]
check-all:
    cargo check --all-targets --all-features

# Run all quality checks (fmt, clippy, test)
[group('Check')]
ci: fmt-check lint test

# Run cargo with all warnings as errors
[group('Check')]
strict:
    RUSTFLAGS="-D warnings" cargo build --all-targets --all-features

# Watch for changes and run tests (requires: cargo-watch)
[group('Test')]
watch-test:
    cargo watch -x test

# Watch for changes and run the project (requires: cargo-watch)
[group('General')]
watch-run:
    cargo watch -x run

# Show outdated dependencies (requires: cargo-outdated. !! Migth not work on some systems)
[group('Dependencies')]
outdated:
    cargo outdated

# Update dependencies
[group('Dependencies')]
update:
    cargo update

# Show the dependency tree
[group('Dependencies')]
tree:
    cargo tree

# Benchmark (if benchmarks exist)
[group('Test')]
bench:
    cargo bench

# Generate code coverage report (requires cargo-tarpaulin)
[group('Test')]
coverage:
    cargo tarpaulin --out Html

# Profile release build
[group('Tools')]
profile:
    cargo build --release
    @echo "Binary built at: target/release/betterfetch"

# Run the todo script
[group('Tools')]
todo *ARGS:
    ./todos.sh {{ ARGS }}
