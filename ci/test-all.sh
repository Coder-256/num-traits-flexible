#!/bin/sh
# Use this locally to emulate CI tests.

run() {
    echo
    echo "Testing with Rust $1..."
    echo
    rustup run "$1" cargo build --verbose
    rustup run "$1" cargo test --verbose
}

run stable || exit $?  # Test Rust stable
run beta || exit $?    # Test Rust beta
run nightly            # Test Rust nightly (allow failure)
