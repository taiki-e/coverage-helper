// Based on https://github.com/dtolnay/proc-macro-hack/blob/0.5.19/src/quote.rs.

use std::iter;

use proc_macro::{TokenStream, TokenTree};

// TODO: remove this #[allow(..)] once https://github.com/rust-lang/rust/pull/97032 backported to beta
#[allow(unknown_lints, unused_macro_rules)]
macro_rules! quote {
    ($($tt:tt)*) => {{
        let mut tokens = ::proc_macro::TokenStream::new();
        quote_each_token!(tokens $($tt)*);
        tokens
    }};
}

// TODO: remove this #[allow(..)] once https://github.com/rust-lang/rust/pull/97032 backported to beta
#[allow(unknown_lints, unused_macro_rules)]
macro_rules! quote_each_token {
    ($tokens:ident # $var:ident $($rest:tt)*) => {
        $crate::quote::Tokens::extend(&mut $tokens, &$var);
        quote_each_token!($tokens $($rest)*);
    };
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
    ($tokens:ident { $($inner:tt)* } $($rest:tt)*) => {
        <::proc_macro::TokenStream as ::std::iter::Extend<_>>::extend(
            &mut $tokens,
            ::std::iter::once(
                ::proc_macro::TokenTree::Group(
                    ::proc_macro::Group::new(
                        ::proc_macro::Delimiter::Brace,
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

pub(crate) trait ToTokens {
    fn to_tokens(&self, tokens: &mut TokenStream);

    fn to_token_stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.to_tokens(&mut tokens);
        tokens
    }
}

impl ToTokens for TokenTree {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(iter::once(self.clone()));
    }
}

impl ToTokens for TokenStream {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.clone());
    }
}
