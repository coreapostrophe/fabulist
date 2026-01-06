pub mod decl;
pub mod expr;
pub mod init;
pub mod stmt;

#[derive(Debug, PartialEq)]
pub struct NodeInfo {
    pub id: usize,
}
