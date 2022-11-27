/*!
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
*/

#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2018_idioms, single_use_lifetimes),
        allow(dead_code, unused_variables)
    )
))]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes, unreachable_pub)]
#![warn(clippy::pedantic)]

// older compilers require explicit `extern crate`.
#[allow(unused_extern_crates)]
extern crate proc_macro;

#[macro_use]
mod error;

#[macro_use]
mod quote;

use std::iter;

use proc_macro::{Span, TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn test(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return format_err!(Span::call_site(), "attribute must be of the form `#[test]`")
            .into_compile_error();
    }
    let mut out = quote! {
        #[cfg_attr(coverage_nightly, no_coverage)]
        #[::core::prelude::v1::test]
    };
    out.extend(input);
    out
}

#[doc(hidden)]
#[proc_macro_attribute]
pub fn __doc_crate_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return format_err!(
            Span::call_site(),
            "attribute must be of the form `#![__doc_crate_attr]`"
        )
        .into_compile_error();
    }
    let mut input = input.into_iter().peekable();
    let mut out = TokenStream::new();
    while let Some(t) = input.next() {
        if let TokenTree::Ident(i) = &t {
            if i.to_string() == "fn" {
                if let Some(TokenTree::Ident(i)) = input.peek() {
                    if i.to_string() == "main" {
                        out.extend(quote! {
                            #[cfg_attr(coverage_nightly, no_coverage)]
                        });
                    }
                }
            }
        }
        out.extend(iter::once(t));
    }
    out
}
