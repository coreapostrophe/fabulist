use fabc_error::{LineCol, Span};
use fabc_parser::ast::{
    expr::{literal::Literal, primitive::Primitive, Expr, Primary},
    NodeInfo,
};

pub fn info(id: usize) -> NodeInfo {
    NodeInfo {
        id,
        span: Span::from((LineCol::new(1, 1), LineCol::new(1, 1))),
    }
}

pub fn number_expr(id: usize, value: f64) -> Expr {
    Expr::Primary {
        info: info(id),
        value: Primary::Literal(Literal::Number {
            info: info(id + 1000),
            value,
        }),
    }
}

pub fn string_expr(id: usize, value: &str) -> Expr {
    Expr::Primary {
        info: info(id),
        value: Primary::Literal(Literal::String {
            info: info(id + 1000),
            value: value.to_string(),
        }),
    }
}

pub fn identifier_expr(id: usize, name: &str) -> Expr {
    Expr::Primary {
        info: info(id),
        value: Primary::Primitive(Primitive::Identifier {
            info: info(id + 1000),
            name: name.to_string(),
        }),
    }
}

pub fn story_identifier_expr(id: usize, name: &str) -> Expr {
    Expr::Primary {
        info: info(id),
        value: Primary::Primitive(Primitive::StoryIdentifier {
            info: info(id + 1000),
            name: name.to_string(),
        }),
    }
}
