// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use std::fmt::{self, Debug, Display};
use pulldown_cmark::{Event, Parser};

use super::Parse;

#[derive(Debug, Fail)]
pub enum TextError {
    #[fail(display = "HTML is not allowed in Text. Found {}", value)]
    DisallowedHtml { value: String },
    #[fail(display = "Inline HTML is not allowed in Text. Found {}", value)]
    DisallowedInlineHtml { value: String },
    #[fail(display = "Validation errors")]
    List(Vec<TextError>),
}

#[derive(Clone, PartialEq)]
pub struct Text(String);

impl Debug for Text {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.debug_tuple("Text").field(&self.0).finish()
    }
}

impl Display for Text {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}

impl Parse for Text {
    type Err = TextError;

    fn parse(s: &str) -> Result<Self, Self::Err> {
        let errors: Vec<TextError> = Parser::new(s)
            .filter_map(|ev| match ev {
                // Escape inline html
                Event::Html(html) => Some(TextError::DisallowedHtml {
                    value: format!("{}", html),
                }),
                Event::InlineHtml(html) => Some(TextError::DisallowedInlineHtml {
                    value: format!("{}", html),
                }),
                _ => None,
            })
            .collect();

        if errors.is_empty() {
            Ok(Text(s.to_owned()))
        } else {
            Err(TextError::List(errors))
        }
    }
}
