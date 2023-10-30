// SPDX-License-Identifier: Apache-2.0 OR MIT

/*!
<!-- tidy:crate-doc:start -->
Helper for <https://github.com/taiki-e/cargo-llvm-cov/issues/123>.

**Note:** coverage-helper 0.2 supports `#[coverage(off)]`.
See coverage-helper 0.1 for versions that support `#[no_coverage]`.

## Usage

Add this to your `Cargo.toml`:

```toml
[dev-dependencies]
coverage-helper = "0.1"
```

And add this to your crate root (`lib.rs` or `main.rs`):

```rust
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
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
#[cfg_attr(coverage_nightly, coverage(off))]
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
        allow(dead_code, unused_variables)
    )
))]
#![forbid(unsafe_code)]

// older compilers require explicit `extern crate`.
#[allow(unused_extern_crates)]
extern crate proc_macro;

#[macro_use]
mod error;

#[macro_use]
mod quote;

use proc_macro::{Span, TokenStream};

#[proc_macro_attribute]
pub fn test(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return format_err!(Span::call_site(), "attribute must be of the form `#[test]`")
            .into_compile_error();
    }
    let mut out = TokenStream::new();
    if cfg!(coverage_helper_has_coverage_attribute) {
        out.extend(quote! { #[cfg_attr(coverage_nightly, coverage(off))] });
    }
    out.extend(quote! { #[::core::prelude::v1::test] });
    out.extend(input);
    out
}
