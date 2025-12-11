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

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Literal::Number(number_literal1) => match other {
                Literal::Number(number_literal2) => number_literal1.value == number_literal2.value,
                _ => false,
            },
            Literal::Boolean(boolean_literal1) => match other {
                Literal::Boolean(boolean_literal2) => {
                    boolean_literal1.value == boolean_literal2.value
                }
                _ => false,
            },
            Literal::String(string_literal1) => match other {
                Literal::String(string_literal2) => string_literal1.value == string_literal2.value,
                _ => false,
            },
            Literal::None(_) => matches!(other, Literal::None(_)),
        }
    }
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Literal::Number(number_literal1) => match other {
                Literal::Number(number_literal2) => {
                    number_literal1.value.partial_cmp(&number_literal2.value)
                }
                _ => None,
            },
            Literal::Boolean(boolean_literal1) => match other {
                Literal::Boolean(boolean_literal2) => {
                    boolean_literal1.value.partial_cmp(&boolean_literal2.value)
                }
                _ => None,
            },
            Literal::String(string_literal1) => match other {
                Literal::String(string_literal2) => {
                    string_literal1.value.partial_cmp(&string_literal2.value)
                }
                _ => None,
            },
            Literal::None(_) => None,
        }
    }
}

#[cfg(test)]
mod expr_overrides_tests {
    use crate::ast::expr::models::{
        BooleanLiteral, Literal, NoneLiteral, NumberLiteral, StringLiteral,
    };

    impl From<f32> for Literal {
        fn from(value: f32) -> Self {
            use crate::error::OwnedSpan;

            Literal::Number(NumberLiteral {
                span: OwnedSpan::default(),
                value,
            })
        }
    }

    impl From<bool> for Literal {
        fn from(value: bool) -> Self {
            use crate::error::OwnedSpan;

            Literal::Boolean(BooleanLiteral {
                span: OwnedSpan::default(),
                value,
            })
        }
    }

    impl From<&str> for Literal {
        fn from(value: &str) -> Self {
            use crate::error::OwnedSpan;

            Literal::String(StringLiteral {
                span: OwnedSpan::default(),
                value: value.to_string(),
            })
        }
    }

    impl From<()> for Literal {
        fn from(_: ()) -> Self {
            use crate::error::OwnedSpan;

            Literal::None(NoneLiteral {
                span: OwnedSpan::default(),
            })
        }
    }

    #[test]
    fn literal_to_num_works() {
        assert_eq!(Literal::from(5.0).to_num().unwrap(), 5.0);
        assert_eq!(Literal::from(true).to_num().unwrap(), 1.0);
        assert_eq!(Literal::from(()).to_num().unwrap(), 0.0);
        assert_eq!(Literal::from("10").to_num().unwrap(), 10.0);

        assert!(Literal::from("hello").to_num().is_err());
        assert_eq!(
            Literal::from("hello").to_num().err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | \n  | ^\n  |\n  = Unable to parse string `hello` to number"
        );
    }

    #[test]
    fn add_literal_works() {
        // number + <literal>
        let result = Literal::from(5.0) + Literal::from(5.0);
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 10.0);

        let result = Literal::from(5.0) + Literal::from(10.0);
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 15.0);

        let result = Literal::from(5.0) + Literal::from(0.0);
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 5.0);

        let result = Literal::from(5.0) + Literal::from(true);
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 6.0);

        // boolean + <literal>
        let result = Literal::from(true) + Literal::from(5.0);
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 6.0);

        let result = Literal::from(true) + Literal::from(0.0);
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 1.0);

        let result = Literal::from(true) + Literal::from(true);
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 2.0);

        let result = Literal::from(true) + Literal::from(10.0);
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 11.0);

        // string + <literal>
        let result = Literal::from("10") + Literal::from(5.0);
        let Literal::String(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a StringLiteral");
        };
        assert_eq!(result.value, "105".to_string());

        let result = Literal::from("10") + Literal::from(true);
        let Literal::String(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a StringLiteral");
        };
        assert_eq!(result.value, "10true".to_string());

        let result = Literal::from("10") + Literal::from(());
        let Literal::String(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a StringLiteral");
        };
        assert_eq!(result.value, "10".to_string());

        let result = Literal::from("10") + Literal::from("10");
        let Literal::String(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a StringLiteral");
        };
        assert_eq!(result.value, "1010".to_string());

        // none + <literal>
        let result = Literal::from(()) + Literal::from(5.0);
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 5.0);

        let result = Literal::from(()) + Literal::from(true);
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 1.0);

        let result = Literal::from(()) + Literal::from("10");
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 10.0);

        let result = Literal::from(()) + Literal::from(());
        let Literal::Number(result) = result.expect("Add failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 0.0);
    }

    #[test]
    fn mul_literal_works() {
        // number * <literal>
        let result = Literal::from(5.0) * Literal::from(5.0);
        let Literal::Number(result) = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 25.0);

        let result = Literal::from(5.0) * Literal::from(true);
        let Literal::Number(result) = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 5.0);

        let result = Literal::from(5.0) * Literal::from(());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | \n  | ^\n  |\n  = Unable to multiply `none` literal"
        );

        let result = Literal::from(5.0) * Literal::from("10");
        let Literal::Number(result) = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 50.0);

        // boolean * <literal>
        let result = Literal::from(true) * Literal::from(5.0);
        let Literal::Number(result) = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 5.0);

        let result = Literal::from(true) * Literal::from(true);
        let Literal::Number(result) = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 1.0);

        let result = Literal::from(true) * Literal::from(());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | \n  | ^\n  |\n  = Unable to multiply `none` literal"
        );

        let result = Literal::from(true) * Literal::from("10");
        let Literal::Number(result) = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 10.0);

        // string * <literal>
        let result = Literal::from("10") * Literal::from(5.0);
        let Literal::Number(result) = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 50.0);

        let result = Literal::from("10") * Literal::from(true);
        let Literal::Number(result) = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 10.0);

        let result = Literal::from("10") * Literal::from(());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | \n  | ^\n  |\n  = Unable to multiply `none` literal"
        );

        let result = Literal::from("10") * Literal::from("10");
        let Literal::Number(result) = result.expect("Mul failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 100.0);

        // none * <literal>
        let result = Literal::from(()) * Literal::from(5.0);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | \n  | ^\n  |\n  = Unable to multiply `none` literal"
        );

        let result = Literal::from(()) * Literal::from(true);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | \n  | ^\n  |\n  = Unable to multiply `none` literal"
        );

        let result = Literal::from(()) * Literal::from("10");
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | \n  | ^\n  |\n  = Unable to multiply `none` literal"
        );

        let result = Literal::from(()) * Literal::from(());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | \n  | ^\n  |\n  = Unable to multiply `none` literal"
        );
    }

    #[test]
    fn div_literal_works() {
        // number / <literal>
        let result = Literal::from(5.0) / Literal::from(5.0);
        let Literal::Number(result) = result.expect("Div failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 1.0);

        let result = Literal::from(5.0) / Literal::from(true);
        let Literal::Number(result) = result.expect("Div failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 5.0);

        let result = Literal::from(5.0) / Literal::from(());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | \n  | ^\n  |\n  = Unable to divide by `none` literal"
        );

        let result = Literal::from(5.0) / Literal::from("10");
        let Literal::Number(result) = result.expect("Div failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 0.5);

        // boolean / <literal>
        let result = Literal::from(true) / Literal::from(5.0);
        let Literal::Number(result) = result.expect("Div failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 0.2);

        let result = Literal::from(true) / Literal::from(true);
        let Literal::Number(result) = result.expect("Div failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 1.0);

        let result = Literal::from(true) / Literal::from(());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | \n  | ^\n  |\n  = Unable to divide by `none` literal"
        );

        let result = Literal::from(true) / Literal::from("10");
        let Literal::Number(result) = result.expect("Div failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 0.1);

        // string / <literal>
        let result = Literal::from("10") / Literal::from(5.0);
        let Literal::Number(result) = result.expect("Div failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 2.0);

        let result = Literal::from("10") / Literal::from(true);
        let Literal::Number(result) = result.expect("Div failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 10.0);

        let result = Literal::from("10") / Literal::from(());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | \n  | ^\n  |\n  = Unable to divide by `none` literal"
        );

        let result = Literal::from("10") / Literal::from("10");
        let Literal::Number(result) = result.expect("Div failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 1.0);

        // none / <literal>
        let result = Literal::from(()) / Literal::from(5.0);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | \n  | ^\n  |\n  = Unable to divide `none` literal"
        );

        let result = Literal::from(()) / Literal::from(true);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | \n  | ^\n  |\n  = Unable to divide `none` literal"
        );

        let result = Literal::from(()) / Literal::from("10");
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | \n  | ^\n  |\n  = Unable to divide `none` literal"
        );

        let result = Literal::from(()) / Literal::from(());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            " --> 1:1\n  |\n1 | \n  | ^\n  |\n  = Unable to divide `none` literal"
        );
    }

    #[test]
    fn sub_literal_works() {
        // number - <literal>
        let result = Literal::from(5.0) - Literal::from(5.0);
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 0.0);

        let result = Literal::from(5.0) - Literal::from(true);
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 4.0);

        let result = Literal::from(5.0) - Literal::from(());
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 5.0);

        let result = Literal::from(5.0) - Literal::from("10");
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, -5.0);

        // boolean - <literal>
        let result = Literal::from(true) - Literal::from(5.0);
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, -4.0);

        let result = Literal::from(true) - Literal::from(true);
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 0.0);

        let result = Literal::from(true) - Literal::from(());
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 1.0);

        let result = Literal::from(true) - Literal::from("10");
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, -9.0);

        // string - <literal>
        let result = Literal::from("10") - Literal::from(5.0);
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 5.0);

        let result = Literal::from("10") - Literal::from(true);
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 9.0);

        let result = Literal::from("10") - Literal::from(());
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 10.0);

        let result = Literal::from("10") - Literal::from("10");
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 0.0);

        // none - <literal>
        let result = Literal::from(()) - Literal::from(5.0);
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, -5.0);

        let result = Literal::from(()) - Literal::from(true);
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, -1.0);

        let result = Literal::from(()) - Literal::from("10");
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, -10.0);

        let result = Literal::from(()) - Literal::from(());
        let Literal::Number(result) = result.expect("Sub failed with an error") else {
            panic!("Expected result to be a NumberLiteral");
        };
        assert_eq!(result.value, 0.0);
    }

    #[test]
    fn eq_literal_works() {
        assert_eq!(Literal::from(5.0), Literal::from(5.0));
        assert_ne!(Literal::from(5.0), Literal::from(10.0));
        assert_eq!(Literal::from(true), Literal::from(true));
        assert_ne!(Literal::from(true), Literal::from(false));
        assert_eq!(Literal::from("hello"), Literal::from("hello"));
        assert_ne!(Literal::from("hello"), Literal::from("world"));
        assert_eq!(Literal::from(()), Literal::from(()));
        assert_ne!(Literal::from(()), Literal::from(5.0));
    }

    #[test]
    fn ord_literal_works() {
        assert!(Literal::from(5.0) < Literal::from(10.0));
        assert!(Literal::from(10.0) > Literal::from(5.0));
        assert!(Literal::from(true) > Literal::from(false));
        assert!(Literal::from(false) < Literal::from(true));
        assert!(Literal::from("apple") < Literal::from("banana"));
        assert!(Literal::from("banana") > Literal::from("apple"));
    }
}
