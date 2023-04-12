# coverage-helper

[![crates.io](https://img.shields.io/crates/v/coverage-helper?style=flat-square&logo=rust)](https://crates.io/crates/coverage-helper)
[![docs.rs](https://img.shields.io/badge/docs.rs-coverage--helper-blue?style=flat-square&logo=docs.rs)](https://docs.rs/coverage-helper)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)](#license)
[![rustc](https://img.shields.io/badge/rustc-1.38+-blue?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![build status](https://img.shields.io/github/actions/workflow/status/taiki-e/coverage-helper/ci.yml?branch=main&style=flat-square&logo=github)](https://github.com/taiki-e/coverage-helper/actions)

<!-- tidy:crate-doc:start -->
Helper for <https://github.com/taiki-e/cargo-llvm-cov/issues/123>.

## Usage

Add this to your `Cargo.toml`:

```toml
[dev-dependencies]
coverage-helper = "0.1"
```

And add this to your crate root (`lib.rs` or `main.rs`):

```rust
#![cfg_attr(coverage_nightly, feature(no_coverage))]
```

## Examples

```rust
use coverage_helper::test;

#[test]
fn my_test() {
    // ...
}
```

Expanded to:

```rust
#[cfg_attr(coverage_nightly, no_coverage)]
#[::core::prelude::v1::test]
fn my_test() {
    // ...
}
```

<!-- tidy:crate-doc:end -->

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
