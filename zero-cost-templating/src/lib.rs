pub mod future_to_stream;

extern crate alloc;

use alloc::borrow::Cow;
use bytes::Bytes;
use std::sync::OnceLock;

pub use future_to_stream::FutureToStream;
pub use future_to_stream::TheStream;
pub use futures::stream::iter;
pub use futures::Stream;
use regex::Captures;
pub use zero_cost_templating_macros::template_stream;

pub struct Unsafe<T: Into<::alloc::borrow::Cow<'static, str>>>(T);

impl<T: Into<::alloc::borrow::Cow<'static, str>>> Unsafe<T> {
    pub fn unsafe_input(input: T) -> Self {
        Self(input)
    }

    pub fn get_unsafe_input(self) -> T {
        self.0
    }
}

pub fn encode_element_text<I: Into<Cow<'static, str>>>(input: I) -> Bytes {
    // https://html.spec.whatwg.org/dev/syntax.html
    // https://www.php.net/manual/en/function.htmlspecialchars.php
    static REGEX: OnceLock<regex::Regex> = OnceLock::new();
    let regex = REGEX.get_or_init(|| regex::Regex::new("[&<]").unwrap());

    let input: Cow<'static, str> = input.into();
    match regex.replace_all(&input, |captures: &Captures| {
        match captures.get(0).unwrap().as_str() {
            "&" => "&amp;",
            "<" => "&lt;",
            _ => unreachable!(),
        }
    }) {
        Cow::Borrowed(_) => match input {
            Cow::Borrowed(value) => Bytes::from(value),
            Cow::Owned(value) => Bytes::from(value),
        },
        Cow::Owned(owned) => Bytes::from(owned),
    }
}

pub fn encode_double_quoted_attribute<I: Into<Cow<'static, str>>>(input: I) -> Bytes {
    // https://html.spec.whatwg.org/dev/dom.html#content-models
    // https://html.spec.whatwg.org/dev/syntax.html
    // https://html.spec.whatwg.org/#escapingString
    // https://html.spec.whatwg.org/
    // In the HTML syntax, authors need only remember to use U+0022 QUOTATION MARK
    // characters (") to wrap the attribute contents and then to escape all U+0026
    // AMPERSAND (&) and U+0022 QUOTATION MARK (") characters, and to specify the
    // sandbox attribute, to ensure safe embedding of content. (And remember to
    // escape ampersands before quotation marks, to ensure quotation marks become
    // &quot; and not &amp;quot;.)
    static REGEX: OnceLock<regex::Regex> = OnceLock::new();
    let regex = REGEX.get_or_init(|| regex::Regex::new("[&\"]").unwrap());

    let input = input.into();
    match regex.replace_all(&input, |captures: &Captures| {
        match captures.get(0).unwrap().as_str() {
            "&" => "&amp;",
            "\"" => "&quot;",
            _ => unreachable!(),
        }
    }) {
        Cow::Borrowed(_) => match input {
            Cow::Borrowed(value) => Bytes::from(value),
            Cow::Owned(value) => Bytes::from(value),
        },
        Cow::Owned(owned) => Bytes::from(owned),
    }
}

#[cfg(test)]
mod tests {}
