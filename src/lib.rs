// SPDX-License-Identifier: Apache-2.0 OR MIT

/*!
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
*/

#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2018_idioms, single_use_lifetimes),
        allow(dead_code, unused_variables, deprecated)
    )
))]
#![forbid(unsafe_code)]
#![allow(deprecated, clippy::test_attr_in_doctest)]

// older compilers require explicit `extern crate`.
#[allow(unused_extern_crates)]
extern crate proc_macro;

#[macro_use]
mod error;

#[macro_use]
mod quote;

use proc_macro::{Span, TokenStream};

#[deprecated(
    since = "0.2.3",
    note = "this crate is deprecated in favor of module-level #[coverage(off)] attribute; \
            see <https://github.com/taiki-e/cargo-llvm-cov?tab=readme-ov-file#exclude-code-from-coverage> \
            for more"
)]
#[proc_macro_attribute]
pub fn test(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return format_err!(Span::call_site(), "attribute must be of the form `#[test]`")
            .into_compile_error();
    }
    let mut out = TokenStream::new();
    if cfg!(coverage_helper_has_coverage_attribute) {
        out.extend(quote! { #[cfg_attr(all(coverage_nightly, test), coverage(off))] });
    }
    out.extend(quote! { #[::core::prelude::v1::test] });
    out.extend(input);
    out
}
