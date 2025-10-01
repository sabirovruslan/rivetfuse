set positional-arguments

# Display help
help:
    just -l

# format code
fmt:
    cargo fmt -- --config imports_granularity=Item --check

fix *args:
    cargo clippy --fix --all-features --tests --allow-dirty "$@"

clippy:
    cargo clippy --all-features --tests "$@"

install:
    rustup toolchain install nightly-aarch64-apple-darwin
    rustup component add rustfmt --toolchain nightly-aarch64-apple-darwin
    cargo fetch

# Run `cargo nextest` since it's faster than `cargo test`, though including
# --no-fail-fast is important to ensure all tests are run.
#
# Run `cargo install cargo-nextest` if you don't have it installed.
test:
    cargo nextest run --no-fail-fast
