# num-traits-flexible

[![crate
](https://img.shields.io/crates/v/num-traits-flexible.svg)
](https://crates.io/crates/num-traits-flexible)
[![documentation
](https://docs.rs/num-traits-flexible/badge.svg)
](https://docs.rs/num-traits-flexible)
[![rust 2018 edition
](https://img.shields.io/badge/rust-2018%20edition-brightgreen.svg)
](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)
[![Travis Status
](https://travis-ci.com/Coder-256/num-traits-flexible.svg?branch=master)
](https://travis-ci.com/Coder-256/num-traits-flexible)

Numeric traits for generic mathematics in Rust. This project is a fork of the
[`num-traits` crate](https://github.com/rust-num/num-traits) created to
implement many postponed breaking changes, along with reorganizing the entire
system of crates into individual operations to allow more flexible generic
requirements.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
num-traits = "1.0"
```

## Releases

Release notes are available in [RELEASES.md](RELEASES.md).

## Compatibility

The `num-traits-flexible` crate works with Rust 2018 and is tested with the
latest stable version.

# Acknowledgements

This project is a fork of the [`num-traits`
crate](https://github.com/rust-num/num-traits). Thanks to everybody who
contributed there! `num-traits-flexible` would have never existed without
their help.
