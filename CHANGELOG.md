# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

Releases may yanked if there is a security bug, a soundness bug, or a regression.

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

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

[Unreleased]: https://github.com/taiki-e/coverage-helper/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/taiki-e/coverage-helper/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/taiki-e/coverage-helper/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/taiki-e/coverage-helper/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/taiki-e/coverage-helper/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/taiki-e/coverage-helper/releases/tag/v0.1.0
