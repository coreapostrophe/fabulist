use std::ops::{Add, Div, Mul, Sub};

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

    pub(crate) fn to_num(&self) -> Result<f32, Box<pest::error::Error<Rule>>> {
        match self {
            Literal::Number(number_literal) => Ok(number_literal.value),
            Literal::Boolean(boolean_literal) => Ok(if boolean_literal.value { 1.0 } else { 0.0 }),
            Literal::None(_) => Ok(0.0),
            Literal::String(string_literal) => {
                let literal_span = string_literal.span.to_owned();
                let literal_value = string_literal.value.to_owned();
                Ok(literal_value.clone().parse::<f32>().map_err(|_| {
                    Box::new(Error::map_custom_error(
                        literal_span,
                        format!("Unable to parse string `{}` to number", literal_value),
                    ))
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
                value: self.to_num().map_err(|err| *err)? + rhs.to_num().map_err(|err| *err)?,
            })),
        }
    }
}

impl Mul for Literal {
    type Output = Result<Literal, pest::error::Error<Rule>>;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Literal::None(factor1) => Err(Error::map_custom_error(
                factor1.span.to_owned(),
                "Unable to multiply `none` literal".to_string(),
            )),
            _ => match rhs {
                Literal::None(factor2) => Err(Error::map_custom_error(
                    factor2.span.to_owned(),
                    "Unable to multiply `none` literal".to_string(),
                )),
                _ => Ok(Literal::Number(NumberLiteral {
                    span: self.span() + rhs.span(),
                    value: self.to_num().map_err(|err| *err)? * rhs.to_num().map_err(|err| *err)?,
                })),
            },
        }
    }
}

impl Sub for Literal {
    type Output = Result<Literal, pest::error::Error<Rule>>;
    fn sub(self, rhs: Self) -> Self::Output {
        Ok(Literal::Number(NumberLiteral {
            span: self.span() + rhs.span(),
            value: self.to_num().map_err(|err| *err)? - rhs.to_num().map_err(|err| *err)?,
        }))
    }
}

impl Div for Literal {
    type Output = Result<Literal, pest::error::Error<Rule>>;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Literal::None(dividend) => Err(Error::map_custom_error(
                dividend.span.to_owned(),
                "Unable to divide `none` literal".to_string(),
            )),
            _ => match rhs {
                Literal::None(divisor) => Err(Error::map_custom_error(
                    divisor.span.to_owned(),
                    "Unable to divide by `none` literal".to_string(),
                )),
                _ => Ok(Literal::Number(NumberLiteral {
                    span: self.span() + rhs.span(),
                    value: self.to_num().map_err(|err| *err)? / rhs.to_num().map_err(|err| *err)?,
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

    fn get_literal_mocks() -> (Literal, Literal, Literal, Literal, Literal) {
        let number = Literal::Number(NumberLiteral {
            span: OwnedSpan {
                input: "5".to_string(),
                start: 0,
                end: 0,
            },
            value: 5.0,
        });
        let boolean = Literal::Boolean(BooleanLiteral {
            span: OwnedSpan {
                input: "true".to_string(),
                start: 0,
                end: 0,
            },
            value: true,
        });
        let none = Literal::None(NoneLiteral {
            span: OwnedSpan {
                input: "none".to_string(),
                start: 0,
                end: 0,
            },
        });
        let string = Literal::String(StringLiteral {
            span: OwnedSpan {
                input: "\"10\"".to_string(),
                start: 0,
                end: 0,
            },
            value: "10".to_string(),
        });
        let alpha_string = Literal::String(StringLiteral {
            span: OwnedSpan {
                input: "\"hello\"".to_string(),
                start: 0,
                end: 0,
            },
            value: "hello".to_string(),
        });

        (number, boolean, none, string, alpha_string)
    }

    #[test]
    fn literal_to_num_works() {
        let (number, boolean, none, string, alpha_string) = get_literal_mocks();

        assert_eq!(number.to_num().unwrap(), 5.0);
        assert_eq!(boolean.to_num().unwrap(), 1.0);
        assert_eq!(none.to_num().unwrap(), 0.0);
        assert_eq!(string.to_num().unwrap(), 10.0);

        assert!(alpha_string.to_num().is_err());
        assert_eq!(
            alpha_string.to_num().err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | \"hello\"\n  | ^\n  |\n  = Unable to parse string `hello` to number"
        );
    }

    #[test]
    fn add_literal_works() {
        let (number, boolean, none, string, _) = get_literal_mocks();

        // number + <literal>
        let result = number.clone() + number.clone();
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 10.0);

        let result = number.clone() + string.clone();
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 15.0);

        let result = number.clone() + none.clone();
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 5.0);

        let result = number.clone() + boolean.clone();
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 6.0);

        // boolean + <literal>
        let result = boolean.clone() + number.clone();
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 6.0);

        let result = boolean.clone() + none.clone();
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 1.0);

        let result = boolean.clone() + boolean.clone();
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 2.0);

        let result = boolean.clone() + string.clone();
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 11.0);

        // string + <literal>
        let result = string.clone() + number.clone();
        let Literal::String(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a StringLiteral");
        };
        assert_eq!(result.value, "105".to_string());

        let result = string.clone() + boolean.clone();
        let Literal::String(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a StringLiteral");
        };
        assert_eq!(result.value, "10true".to_string());

        let result = string.clone() + none.clone();
        let Literal::String(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a StringLiteral");
        };
        assert_eq!(result.value, "10".to_string());

        let result = string.clone() + string.clone();
        let Literal::String(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a StringLiteral");
        };
        assert_eq!(result.value, "1010".to_string());

        // none + <literal>
        let result = none.clone() + number.clone();
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 5.0);

        let result = none.clone() + boolean.clone();
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 1.0);

        let result = none.clone() + string.clone();
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a StringLiteral");
        };
        assert_eq!(result.value, 10.0);

        let result = none.clone() + none.clone();
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 0.0);
    }

    #[test]
    fn mul_literal_works() {
        let (number, boolean, none, string, _) = get_literal_mocks();

        // number * <literal>
        let result = number.clone() * number.clone();
        let Literal::Number(result) = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 25.0);

        let result = number.clone() * boolean.clone();
        let Literal::Number(result) = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 5.0);

        let result = number.clone() * none.clone();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | none\n  | ^\n  |\n  = Unable to multiply `none` literal"
        );

        let result = number.clone() * string.clone();
        let Literal::Number(result) = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 50.0);

        // boolean * <literal>
        let result = boolean.clone() * number.clone();
        let Literal::Number(result) = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 5.0);

        let result = boolean.clone() * boolean.clone();
        let Literal::Number(result) = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 1.0);

        let result = boolean.clone() * none.clone();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | none\n  | ^\n  |\n  = Unable to multiply `none` literal"
        );

        let result = boolean.clone() * string.clone();
        let Literal::Number(result) = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 10.0);

        // string * <literal>
        let result = string.clone() * number.clone();
        let Literal::Number(result) = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 50.0);

        let result = string.clone() * boolean.clone();
        let Literal::Number(result) = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 10.0);

        let result = string.clone() * none.clone();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | none\n  | ^\n  |\n  = Unable to multiply `none` literal"
        );

        let result = string.clone() * string.clone();
        let Literal::Number(result) = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 100.0);

        // none * <literal>
        let result = none.clone() * number.clone();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | none\n  | ^\n  |\n  = Unable to multiply `none` literal"
        );

        let result = none.clone() * boolean.clone();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | none\n  | ^\n  |\n  = Unable to multiply `none` literal"
        );

        let result = none.clone() * string.clone();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | none\n  | ^\n  |\n  = Unable to multiply `none` literal"
        );

        let result = none.clone() * none.clone();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | none\n  | ^\n  |\n  = Unable to multiply `none` literal"
        );
    }

    #[test]
    fn div_literal_works() {
        let (number, boolean, none, string, _) = get_literal_mocks();

        // number / <literal>
        let result = number.clone() / number.clone();
        let Literal::Number(result) = result.expect("Div failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 1.0);

        let result = number.clone() / boolean.clone();
        let Literal::Number(result) = result.expect("Div failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 5.0);

        let result = number.clone() / none.clone();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | none\n  | ^\n  |\n  = Unable to divide by `none` literal"
        );

        let result = number.clone() / string.clone();
        let Literal::Number(result) = result.expect("Div failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 0.5);

        // boolean / <literal>
        let result = boolean.clone() / number.clone();
        let Literal::Number(result) = result.expect("Div failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 0.2);

        let result = boolean.clone() / boolean.clone();
        let Literal::Number(result) = result.expect("Div failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 1.0);

        let result = boolean.clone() / none.clone();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | none\n  | ^\n  |\n  = Unable to divide by `none` literal"
        );

        let result = boolean.clone() / string.clone();
        let Literal::Number(result) = result.expect("Div failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 0.1);

        // string / <literal>
        let result = string.clone() / number.clone();
        let Literal::Number(result) = result.expect("Div failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 2.0);

        let result = string.clone() / boolean.clone();
        let Literal::Number(result) = result.expect("Div failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 10.0);

        let result = string.clone() / none.clone();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | none\n  | ^\n  |\n  = Unable to divide by `none` literal"
        );

        let result = string.clone() / string.clone();
        let Literal::Number(result) = result.expect("Div failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 1.0);

        // none / <literal>
        let result = none.clone() / number.clone();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | none\n  | ^\n  |\n  = Unable to divide `none` literal"
        );

        let result = none.clone() / boolean.clone();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | none\n  | ^\n  |\n  = Unable to divide `none` literal"
        );

        let result = none.clone() / string.clone();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | none\n  | ^\n  |\n  = Unable to divide `none` literal"
        );

        let result = none.clone() / none.clone();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | none\n  | ^\n  |\n  = Unable to divide `none` literal"
        );
    }

    #[test]
    fn sub_literal_works() {
        let (number, boolean, none, string, _) = get_literal_mocks();

        // number - <literal>
        let result = number.clone() - number.clone();
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 0.0);

        let result = number.clone() - boolean.clone();
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 4.0);

        let result = number.clone() - none.clone();
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 5.0);

        let result = number.clone() - string.clone();
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, -5.0);

        // boolean - <literal>
        let result = boolean.clone() - number.clone();
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, -4.0);

        let result = boolean.clone() - boolean.clone();
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 0.0);

        let result = boolean.clone() - none.clone();
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 1.0);

        let result = boolean.clone() - string.clone();
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, -9.0);

        // string - <literal>
        let result = string.clone() - number.clone();
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 5.0);

        let result = string.clone() - boolean.clone();
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 9.0);

        let result = string.clone() - none.clone();
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 10.0);

        let result = string.clone() - string.clone();
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 0.0);

        // none - <literal>
        let result = none.clone() - number.clone();
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, -5.0);

        let result = none.clone() - boolean.clone();
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, -1.0);

        let result = none.clone() - string.clone();
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, -10.0);

        let result = none.clone() - none.clone();
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 0.0);
    }
}
