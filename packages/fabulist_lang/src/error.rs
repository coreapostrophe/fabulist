use crate::parser::Rule;

#[derive(thiserror::Error, Debug)]
pub enum Error {}

impl Error {
    pub fn map_custom_error(
        span: pest::Span,
        message: impl Into<String>,
    ) -> pest::error::Error<Rule> {
        pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError {
                message: message.into(),
            },
            span,
        )
    }
}
