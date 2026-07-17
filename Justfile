# Run `cargo install just`. Then run `just` to list available recipes.

default:
  just --list

# Perform the one-time local setup required for this crate.
once:
  just enable-wasm
  just install-tools

# Install the tools this crate depends on for local development.
install-tools:
  cargo install just
  cargo install cargo-leptos
  cargo install wasm-bindgen-cli --version 0.2.125 --locked
  cargo install cargo-audit
  cargo install cargo-deny
  cargo install cargo-semver-checks
  cargo install leptosfmt

# Enable the WASM target required by `cargo leptos` and the wasm compile check.
enable-wasm:
  rustup target add wasm32-unknown-unknown

# Format the crate.
fmt:
  cargo fmt --all

# Run format checks.
fmt-check:
  cargo fmt --all --check

# Format Leptos code.
leptosfmt:
  leptosfmt ./testing/test-app/*

# Run clippy with the default checked CSS API.
clippy:
  cargo clippy --all-targets -- -D warnings

# Run clippy on the minimal unchecked-only feature shape.
clippy-no-default:
  cargo clippy --all-targets --no-default-features -- -D warnings

# Type-check the default checked CSS API.
check:
  cargo check --all-targets

# Type-check the minimal unchecked-only feature shape.
check-no-default:
  cargo check --all-targets --no-default-features

# Type-check the checked CSS API with Leptos' nightly closure implementations.
check-nightly:
  cargo +nightly check --all-targets --all-features --locked

# Type-check the unchecked-only API with Leptos' nightly closure implementations.
check-nightly-no-default:
  cargo +nightly check --all-targets --no-default-features --features nightly --locked

# Verify the default checked CSS API compiles for wasm32-unknown-unknown.
check-wasm:
  cargo check --target wasm32-unknown-unknown --locked

# Verify the minimal unchecked-only feature shape compiles for wasm32-unknown-unknown.
check-wasm-no-default:
  cargo check --target wasm32-unknown-unknown --no-default-features --locked

# Run unit tests, integration tests, and doctests for the default feature set.
test:
  cargo test

# Run only the crate unit tests with default features.
test-lib:
  cargo test --lib

# Run only the crate unit tests with default features disabled.
test-lib-no-default:
  cargo test --lib --no-default-features

# Run only the crate doc tests with default features.
test-doc:
  cargo test --doc

# Run only the crate doc tests with default features disabled.
test-doc-no-default:
  cargo test --doc --no-default-features

# Run the typed-css integration test suite (`tests/typed_css.rs`).
test-typed-css:
  cargo test --test typed_css

# Build documentation with rustdoc warnings denied (matches CI).
doc:
  RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --locked

# Serve the crate-local test app for manual inspection.
serve-test-app:
  cd ./testing/test-app && cargo leptos serve

# Run the Chrome-based browser integration test headlessly.
browser-test:
  cargo test --test browser_test -- --nocapture

# Run the Chrome-based browser integration test with a visible browser.
browser-test-visible:
  BROWSER_TEST_VISIBLE=1 cargo test --test browser_test -- --nocapture

# Start the test app and pause before running browser assertions.
browser-test-pause:
  BROWSER_TEST_VISIBLE=1 BROWSER_TEST_PAUSE=1 cargo test --test browser_test -- --nocapture

# Scan Cargo.lock against the RustSec advisory database.
audit:
  cargo audit

# Run cargo-deny's supply-chain checks (advisories, bans, sources).
deny:
  cargo deny check

# Detect breaking public-API changes vs. the latest published release.
semver-check:
  cargo semver-checks

# Clean build artifacts for this crate and its local test app.
clean:
  cargo clean
  cargo clean --manifest-path ./testing/test-app/Cargo.toml

# Run the most important verification commands for this crate. Covers both feature
# configurations (default checked CSS and no-default unchecked-only) since both public shapes
# must compile cleanly.
verify:
  just fmt-check
  just check
  just check-no-default
  just check-wasm
  just check-wasm-no-default
  just clippy
  just clippy-no-default
  just test-lib
  just test-lib-no-default
  just test-doc
  just test-doc-no-default
  just test-typed-css
  just browser-test
  just doc
