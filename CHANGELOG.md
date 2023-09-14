# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

**Note:** coverage-helper 0.2 supports `#[coverage(off)]`.
See coverage-helper 0.1 for versions that support `#[no_coverage]`.

- Use [`#[coverage(off)]`](https://github.com/rust-lang/rust/pull/114656) instead of `#[no_coverage]`. ([#4](https://github.com/taiki-e/coverage-helper/pull/4))

## [0.1.1] - 2023-09-13

- Prepare for [renaming of `#[no_coverage]` in future nightly](https://github.com/rust-lang/rust/pull/114656). ([#3](https://github.com/taiki-e/coverage-helper/pull/3))

## [0.1.0] - 2022-05-29

Initial release

[Unreleased]: https://github.com/taiki-e/coverage-helper/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/taiki-e/coverage-helper/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/taiki-e/coverage-helper/releases/tag/v0.1.0
