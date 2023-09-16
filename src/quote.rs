// SPDX-License-Identifier: Apache-2.0 OR MIT

// Based on https://github.com/dtolnay/proc-macro-hack/blob/0.5.19/src/quote.rs.

macro_rules! quote {
    ($($tt:tt)*) => {{
        let mut tokens = ::proc_macro::TokenStream::new();
        quote_each_token!(tokens $($tt)*);
        tokens
    }};
}

macro_rules! quote_each_token {
    ($tokens:ident $ident:ident $($rest:tt)*) => {
        <::proc_macro::TokenStream as ::std::iter::Extend<_>>::extend(
            &mut $tokens,
            ::std::iter::once(
                ::proc_macro::TokenTree::Ident(
                    ::proc_macro::Ident::new(
                        stringify!($ident),
                        ::proc_macro::Span::call_site(),
                    ),
                ),
            ),
        );
        quote_each_token!($tokens $($rest)*);
    };
    ($tokens:ident ( $($inner:tt)* ) $($rest:tt)*) => {
        <::proc_macro::TokenStream as ::std::iter::Extend<_>>::extend(
            &mut $tokens,
            ::std::iter::once(
                ::proc_macro::TokenTree::Group(
                    ::proc_macro::Group::new(
                        ::proc_macro::Delimiter::Parenthesis,
                        quote!($($inner)*),
                    ),
                ),
            ),
        );
        quote_each_token!($tokens $($rest)*);
    };
    ($tokens:ident [ $($inner:tt)* ] $($rest:tt)*) => {
        <::proc_macro::TokenStream as ::std::iter::Extend<_>>::extend(
            &mut $tokens,
            ::std::iter::once(
                ::proc_macro::TokenTree::Group(
                    ::proc_macro::Group::new(
                        ::proc_macro::Delimiter::Bracket,
                        quote!($($inner)*),
                    ),
                ),
            ),
        );
        quote_each_token!($tokens $($rest)*);
    };
    ($tokens:ident $punct:tt $($rest:tt)*) => {
        <::proc_macro::TokenStream as ::std::iter::Extend<_>>::extend(
            &mut $tokens,
            stringify!($punct).parse::<::proc_macro::TokenStream>(),
        );
        quote_each_token!($tokens $($rest)*);
    };
    ($tokens:ident) => {};
}
