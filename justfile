# Mourhhold task runner. Install `just`: https://github.com/casey/just
# List recipes with `just` or `just --list`.

set shell := ["bash", "-cu"]

# Show available recipes.
default:
    @just --list

# Run the game with fast-iteration dynamic linking.
run:
    cargo run --features dev

# Run the game in optimized release mode.
run-release:
    cargo run --release

# Format the whole workspace.
fmt:
    cargo fmt --all

# Verify formatting without writing changes (CI-style).
fmt-check:
    cargo fmt --all -- --check

# Lint with warnings promoted to errors.
lint:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Run the test suite (uses nextest if installed, else cargo test).
test:
    @if command -v cargo-nextest >/dev/null 2>&1; then \
        cargo nextest run --workspace --all-features; \
    else \
        cargo test --workspace --all-features; \
    fi

# Generate an HTML+lcov coverage report (requires cargo-llvm-cov).
coverage:
    cargo llvm-cov --workspace --all-features --html
    cargo llvm-cov report --lcov --output-path lcov.info

# Supply-chain + license + advisory checks (requires cargo-deny).
deny:
    cargo deny check

# Find unused dependencies (requires cargo-machete).
machete:
    cargo machete

# Build the rustdoc site for all workspace crates.
docs:
    cargo doc --workspace --no-deps --all-features

# Run the full local quality gate before pushing.
ci: fmt-check lint test deny

# Install the developer tooling this project expects.
setup-tools:
    cargo install cargo-nextest cargo-llvm-cov cargo-deny cargo-machete
    rustup component add llvm-tools-preview
    @echo "Now run: pre-commit install --install-hooks && pre-commit install --hook-type commit-msg"
