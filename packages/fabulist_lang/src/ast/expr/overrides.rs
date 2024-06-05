use std::ops::Add;

use crate::{error::Error, parser::Rule};

use super::models::{BooleanLiteral, Literal, NoneLiteral, NumberLiteral, StringLiteral};

impl BooleanLiteral {
    pub(crate) fn to_f32(&self) -> f32 {
        if self.value {
            1.0
        } else {
            0.0
        }
    }
}

impl StringLiteral {
    pub(crate) fn to_f32(&self) -> Result<f32, Box<pest::error::Error<Rule>>> {
        self.value.parse::<f32>().map_err(|_| {
            Box::new(Error::map_custom_error(
                self.span.to_owned(),
                format!("Unable to parse string `{}` to number", self.value),
            ))
        })
    }
}

impl Add for Literal {
    type Output = Result<Literal, pest::error::Error<Rule>>;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Literal::Number(addend1) => match rhs {
                Literal::Number(addend2) => Ok(Literal::Number(NumberLiteral {
                    span: addend1.span.to_owned() + addend2.span.to_owned(),
                    value: addend1.value + addend2.value,
                })),
                Literal::Boolean(addend2) => Ok(Literal::Number(NumberLiteral {
                    span: addend1.span.to_owned() + addend2.span.to_owned(),
                    value: addend1.value + addend2.to_f32(),
                })),
                Literal::None(addend2) => Ok(Literal::Number(NumberLiteral {
                    span: addend1.span.to_owned() + addend2.span.to_owned(),
                    value: addend1.value,
                })),
                Literal::String(addend2) => Ok(Literal::Number(NumberLiteral {
                    span: addend1.span.to_owned() + addend2.span.to_owned(),
                    value: addend1.value + addend2.to_f32().map_err(|err| *err)?,
                })),
            },
            Literal::None(addend1) => match rhs {
                Literal::Number(addend2) => Ok(Literal::Number(NumberLiteral {
                    span: addend1.span.to_owned() + addend2.span.to_owned(),
                    value: addend2.value,
                })),
                Literal::Boolean(addend2) => Ok(Literal::Boolean(BooleanLiteral {
                    span: addend1.span.to_owned() + addend2.span.to_owned(),
                    value: addend2.value,
                })),
                Literal::None(addend2) => Ok(Literal::None(NoneLiteral {
                    span: addend1.span.to_owned() + addend2.span.to_owned(),
                })),
                Literal::String(addend2) => Ok(Literal::String(StringLiteral {
                    span: addend1.span.to_owned() + addend2.span.to_owned(),
                    value: addend2.value,
                })),
            },
            Literal::Boolean(addend1) => match rhs {
                Literal::Number(addend2) => Ok(Literal::Number(NumberLiteral {
                    span: addend1.span.to_owned() + addend2.span.to_owned(),
                    value: addend1.to_f32() + addend2.value,
                })),
                Literal::Boolean(addend2) => Ok(Literal::Number(NumberLiteral {
                    span: addend1.span.to_owned() + addend2.span.to_owned(),
                    value: addend1.to_f32() + addend2.to_f32(),
                })),
                Literal::None(addend2) => Ok(Literal::Boolean(BooleanLiteral {
                    span: addend1.span.to_owned() + addend2.span.to_owned(),
                    value: addend1.value,
                })),
                Literal::String(addend2) => Ok(Literal::Number(NumberLiteral {
                    span: addend1.span.to_owned() + addend2.span.to_owned(),
                    value: addend1.to_f32() + addend2.to_f32().map_err(|err| *err)?,
                })),
            },
            Literal::String(addend1) => match rhs {
                Literal::Number(addend2) => Ok(Literal::Number(NumberLiteral {
                    span: addend1.span.to_owned() + addend2.span.to_owned(),
                    value: addend1.to_f32().map_err(|err| *err)? + addend2.value,
                })),
                Literal::Boolean(addend2) => Ok(Literal::Number(NumberLiteral {
                    span: addend1.span.to_owned() + addend2.span.to_owned(),
                    value: addend1.to_f32().map_err(|err| *err)? + addend2.to_f32(),
                })),
                Literal::None(addend2) => Ok(Literal::Number(NumberLiteral {
                    span: addend1.span.to_owned() + addend2.span.to_owned(),
                    value: addend1.to_f32().map_err(|err| *err)?,
                })),
                Literal::String(addend2) => Ok(Literal::String(StringLiteral {
                    span: addend1.span.to_owned() + addend2.span.to_owned(),
                    value: addend1.value + &addend2.value,
                })),
            },
        }
    }
}
