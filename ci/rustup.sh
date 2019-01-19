#!/bin/sh
# Use rustup to locally run the same suite of tests as .travis.yml.
# (You should first install/update stable, beta, and nightly.)

set -x
export TRAVIS_RUST_VERSION

run() {
    rustup run "$TRAVIS_RUST_VERSION" ./ci/test_full.sh
}

TRAVIS_RUST_VERSION=stable run || exit $?  # Test Rust stable
TRAVIS_RUST_VERSION=beta run || exit $?    # Test Rust beta
TRAVIS_RUST_VERSION=nightly run            # Test Rust nightly (allow failure)