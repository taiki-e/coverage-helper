# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

Releases may yanked if there is a security bug, a soundness bug, or a regression.

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

## [0.2.4] - 2025-01-18

- Update documentation to reflect the revert of `coverage_attribute` stabilization.

## [0.2.3] - 2024-12-18

**Note: This crate is now deprecated in favor of the pattern that is [recommended in the cargo-llvm-cov documentation](https://github.com/taiki-e/cargo-llvm-cov?tab=readme-ov-file#exclude-code-from-coverage).**

> If you want to ignore all `#[test]`-related code, you can use module-level `#[coverage(off)]` attribute:
>
> ```rust
> #[cfg(test)]
> #[cfg_attr(coverage, coverage(off))]
> mod tests {
>     // ...
> }
> ```
>
> cargo-llvm-cov excludes code contained in the directory named `tests` from the report by default, so you can also use it instead of `#[coverage(off)]` attribute.

`#[coverage(off)]` attribute has been stabilized in [rust-lang/rust#130766](https://github.com/rust-lang/rust/pull/130766) (will be included in Rust 1.85).

## [0.2.2] - 2024-04-21

- Update documentation to recommend using `#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]` instead of `#![cfg_attr(coverage_nightly, feature(coverage_attribute))]`.

## [0.2.1] - 2024-04-21

- Respect `RUSTC_WORKSPACE_WRAPPER` when checking availability of `#[coverage(off)]`.

## [0.2.0] - 2023-09-14

**Note:** coverage-helper 0.2 supports `#[coverage(off)]`.
See coverage-helper 0.1 for versions that support `#[no_coverage]`.

- Use [`#[coverage(off)]`](https://github.com/rust-lang/rust/pull/114656) instead of `#[no_coverage]`. ([#4](https://github.com/taiki-e/coverage-helper/pull/4))

## [0.1.1] - 2023-09-13

- Prepare for [renaming of `#[no_coverage]` in future nightly](https://github.com/rust-lang/rust/pull/114656). ([#3](https://github.com/taiki-e/coverage-helper/pull/3))

## [0.1.0] - 2022-05-29

Initial release

[Unreleased]: https://github.com/taiki-e/coverage-helper/compare/v0.2.4...HEAD
[0.2.4]: https://github.com/taiki-e/coverage-helper/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/taiki-e/coverage-helper/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/taiki-e/coverage-helper/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/taiki-e/coverage-helper/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/taiki-e/coverage-helper/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/taiki-e/coverage-helper/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/taiki-e/coverage-helper/releases/tag/v0.1.0
