use std::iter::FromIterator;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

// TODO: remove this #[allow(..)] once https://github.com/rust-lang/rust/pull/97032 backported to beta
#[allow(unknown_lints, unused_macro_rules)]
macro_rules! format_err {
    ($span:expr, $msg:expr $(,)*) => {
        crate::error::Error::new($span, String::from($msg))
    };
    ($span:expr, $($tt:tt)*) => {
        format_err!($span, format!($($tt)*))
    };
}

pub(crate) struct Error {
    span: Span,
    msg: String,
}

impl Error {
    pub(crate) fn new(span: Span, msg: String) -> Self {
        Self { span, msg }
    }

    // https://github.com/dtolnay/syn/blob/1.0.39/src/error.rs#L218-L237
    pub(crate) fn into_compile_error(self) -> TokenStream {
        // compile_error!($msg)
        TokenStream::from_iter(vec![
            TokenTree::Ident(Ident::new("compile_error", self.span)),
            TokenTree::Punct({
                let mut punct = Punct::new('!', Spacing::Alone);
                punct.set_span(self.span);
                punct
            }),
            TokenTree::Group({
                let mut group = Group::new(Delimiter::Brace, {
                    TokenStream::from_iter(vec![TokenTree::Literal({
                        let mut string = Literal::string(&self.msg);
                        string.set_span(self.span);
                        string
                    })])
                });
                group.set_span(self.span);
                group
            }),
        ])
    }
}
