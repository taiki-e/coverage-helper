# coverage-helper

[![crates.io](https://img.shields.io/crates/v/coverage-helper?style=flat-square&logo=rust)](https://crates.io/crates/coverage-helper)
[![docs.rs](https://img.shields.io/badge/docs.rs-coverage--helper-blue?style=flat-square&logo=docs.rs)](https://docs.rs/coverage-helper)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)](#license)
[![msrv](https://img.shields.io/badge/msrv-1.38-blue?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![github actions](https://img.shields.io/github/actions/workflow/status/taiki-e/coverage-helper/ci.yml?branch=main&style=flat-square&logo=github)](https://github.com/taiki-e/coverage-helper/actions)
![maintenance-status](https://img.shields.io/badge/maintenance-deprecated-red?style=flat-square)

<!-- tidy:crate-doc:start -->

**Note: This crate is now deprecated in favor of the pattern that is [recommended in the cargo-llvm-cov documentation](https://github.com/taiki-e/cargo-llvm-cov?tab=readme-ov-file#exclude-code-from-coverage).**

> If you want to ignore all `#[test]`-related code, you can use module-level `#[coverage(off)]` attribute:
>
> ```rust
> #![cfg_attr(coverage_nightly, feature(coverage_attribute))]
>
> #[cfg(test)]
> #[cfg_attr(coverage_nightly, coverage(off))]
> mod tests {
>     // ...
> }
> ```
>
> cargo-llvm-cov excludes code contained in the directory named `tests` from the report by default, so you can also use it instead of `#[coverage(off)]` attribute.

---

Helper for <https://github.com/taiki-e/cargo-llvm-cov/issues/123>.

**Note:** coverage-helper 0.2 supports `#[coverage(off)]`.
See coverage-helper 0.1 for versions that support `#[no_coverage]`.

## Usage

Add this to your `Cargo.toml`:

```toml
[dev-dependencies]
coverage-helper = "0.2"
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
#[cfg_attr(all(coverage_nightly, test), coverage(off))]
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
