# Cargo run
default:
	cargo run

# Build the project in release mode
build:
    cargo build --release

# Run the project from source
run:
	cargo run

# Install the binary globally
install:
    cargo install --path .

# Uninstall the globally installed binary
uninstall:
    cargo uninstall betterfetch

# Lint with Clippy
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Format the code
fmt:
    cargo fmt --all

# Clean build artifacts
clean:
    cargo clean
