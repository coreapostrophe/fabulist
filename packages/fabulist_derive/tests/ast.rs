use fabulist_derive::SyntaxTree;

#[derive(SyntaxTree, Debug)]
pub enum Expr {
    #[production(left: Expr, right: Expr)]
    Binary(Box<BinaryExpr>),
}

fn main() {}
