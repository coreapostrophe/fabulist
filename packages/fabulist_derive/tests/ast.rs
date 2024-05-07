use fabulist_derive::SyntaxTree;

#[derive(SyntaxTree, Debug, Clone)]
pub enum Expr {
    #[production(left: Expr, right: Vec<Expr>)]
    Binary(Box<BinaryExpr>),
}

fn main() {}
