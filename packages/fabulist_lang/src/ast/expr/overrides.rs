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

#[cfg(test)]
mod expr_overrides_tests {

    use crate::{
        ast::expr::models::{BooleanLiteral, Literal, NoneLiteral, NumberLiteral, StringLiteral},
        error::OwnedSpan,
    };

    #[test]
    fn add_literal_works() {
        let number = Literal::Number(NumberLiteral {
            span: OwnedSpan {
                input: "".to_string(),
                start: 0,
                end: 0,
            },
            value: 5.0,
        });
        let boolean = Literal::Boolean(BooleanLiteral {
            span: OwnedSpan {
                input: "".to_string(),
                start: 0,
                end: 0,
            },
            value: true,
        });
        let none = Literal::None(NoneLiteral {
            span: OwnedSpan {
                input: "".to_string(),
                start: 0,
                end: 0,
            },
        });
        let string = Literal::String(StringLiteral {
            span: OwnedSpan {
                input: "".to_string(),
                start: 0,
                end: 0,
            },
            value: "10".to_string(),
        });

        // number + <literal>
        let result = number.clone() + number.clone();
        if let Literal::Number(result) = result.expect("Add failed with an error") {
            assert_eq!(result.value, 10.0);
        };
        let result = number.clone() + string.clone();
        if let Literal::Number(result) = result.expect("Add failed with an error") {
            assert_eq!(result.value, 15.0);
        };
        let result = number.clone() + none.clone();
        if let Literal::Number(result) = result.expect("Add failed with an error") {
            assert_eq!(result.value, 5.0);
        };
        let result = number.clone() + boolean.clone();
        if let Literal::Number(result) = result.expect("Add failed with an error") {
            assert_eq!(result.value, 6.0);
        };

        // boolean + <literal>
        let result = boolean.clone() + number.clone();
        if let Literal::Number(result) = result.expect("Add failed with an error") {
            assert_eq!(result.value, 6.0);
        };
        let result = boolean.clone() + none.clone();
        if let Literal::Number(result) = result.expect("Add failed with an error") {
            assert_eq!(result.value, 1.0);
        };
        let result = boolean.clone() + boolean.clone();
        if let Literal::Number(result) = result.expect("Add failed with an error") {
            assert_eq!(result.value, 2.0);
        };
        let result = boolean.clone() + string.clone();
        if let Literal::Number(result) = result.expect("Add failed with an error") {
            assert_eq!(result.value, 11.0);
        };

        // string + <literal>
        let result = string.clone() + number.clone();
        if let Literal::Number(result) = result.expect("Add failed with an error") {
            assert_eq!(result.value, 15.0);
        };
        let result = string.clone() + boolean.clone();
        if let Literal::Number(result) = result.expect("Add failed with an error") {
            assert_eq!(result.value, 11.0);
        };
        let result = string.clone() + none.clone();
        if let Literal::String(result) = result.expect("Add failed with an error") {
            assert_eq!(result.value, "10".to_string());
        };
        let result = string.clone() + string.clone();
        if let Literal::String(result) = result.expect("Add failed with an error") {
            assert_eq!(result.value, "1010".to_string());
        };

        // none + <literal>
        let result = none.clone() + number.clone();
        if let Literal::Number(result) = result.expect("Add failed with an error") {
            assert_eq!(result.value, 5.0);
        };
        let result = none.clone() + boolean.clone();
        if let Literal::Boolean(result) = result.expect("Add failed with an error") {
            assert_eq!(result.value, true);
        };
        let result = none.clone() + string.clone();
        if let Literal::String(result) = result.expect("Add failed with an error") {
            assert_eq!(result.value, "10".to_string());
        };
        let result = none.clone() + none.clone();
        if let Literal::None(_) = result.expect("Add failed with an error") {
            assert!(true)
        };
    }
}
