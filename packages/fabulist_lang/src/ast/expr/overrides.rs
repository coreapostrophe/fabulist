use std::ops::Add;

use crate::{
    error::{Error, OwnedSpan},
    parser::Rule,
};

use super::models::{Literal, NumberLiteral, StringLiteral};

impl Literal {
    pub(crate) fn span(&self) -> OwnedSpan {
        match self {
            Literal::Number(number_literal) => number_literal.span.to_owned(),
            Literal::Boolean(boolean_literal) => boolean_literal.span.to_owned(),
            Literal::String(string_literal) => string_literal.span.to_owned(),
            Literal::None(none_literal) => none_literal.span.to_owned(),
        }
    }

    pub(crate) fn to_num(&self) -> Result<f32, pest::error::Error<Rule>> {
        match self {
            Literal::Number(number_literal) => Ok(number_literal.value),
            Literal::Boolean(boolean_literal) => Ok(if boolean_literal.value { 1.0 } else { 0.0 }),
            Literal::None(_) => Ok(0.0),
            Literal::String(string_literal) => {
                let literal_span = string_literal.span.to_owned();
                let literal_value = string_literal.value.to_owned();
                Ok(literal_value.clone().parse::<f32>().map_err(|_| {
                    Error::map_custom_error(
                        literal_span,
                        format!("Unable to parse string `{}` to number", literal_value),
                    )
                })?)
            }
        }
    }
}

impl Add for Literal {
    type Output = Result<Literal, pest::error::Error<Rule>>;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Literal::String(addend1) => match rhs {
                Literal::Number(addend2) => Ok(Literal::String(StringLiteral {
                    span: addend1.span.to_owned() + addend2.span.to_owned(),
                    value: format!("{}{}", addend1.value, addend2.value),
                })),
                Literal::String(addend2) => Ok(Literal::String(StringLiteral {
                    span: addend1.span.to_owned() + addend2.span.to_owned(),
                    value: format!("{}{}", addend1.value, addend2.value),
                })),
                Literal::Boolean(addend2) => Ok(Literal::String(StringLiteral {
                    span: addend1.span.to_owned() + addend2.span.to_owned(),
                    value: format!("{}{}", addend1.value, addend2.value),
                })),
                Literal::None(addend2) => Ok(Literal::String(StringLiteral {
                    span: addend1.span.to_owned() + addend2.span.to_owned(),
                    value: addend1.value,
                })),
            },
            _ => Ok(Literal::Number(NumberLiteral {
                span: self.span() + rhs.span(),
                value: self.to_num()? + rhs.to_num()?,
            })),
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
