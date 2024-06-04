use crate::{error::Error, parser::Rule};

use super::models::Literal;

impl Literal {
    pub fn to_f32(&self) -> Result<f32, pest::error::Error<Rule>> {
        match self {
            Literal::Boolean(boolean) => Ok(if boolean.value { 1.0 } else { 0.0 }),
            Literal::Number(number) => Ok(number.value),
            Literal::None(_) => Ok(0.0),
            Literal::String(string) => {
                let span = string.span.to_owned();
                let value = string.value.to_owned();
                value.parse::<f32>().map_err(|_| {
                    Error::map_custom_error(span, format!("Unable to parse `{}` to `f32`", value))
                })
            }
        }
    }
}
