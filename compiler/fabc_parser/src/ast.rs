use std::any::Any;

use crate::ast::{expr::Expr, stmt::Stmt, story::part::Part};

pub mod decl;
pub mod expr;
pub mod stmt;
pub mod story;

#[derive(Debug, PartialEq)]
pub struct NodeId {
    pub idx: usize,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Expr(Expr),
    Stmt(Stmt),
    Part(Part),
}

#[derive(Default)]
pub struct NodeCollection {
    pub nodes: Vec<Node>,
}

impl NodeCollection {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_node(&mut self, node: Node) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(node);
        NodeId { idx: id }
    }
    pub fn get_node(&self, node_id: &NodeId) -> Option<&Node> {
        self.nodes.get(node_id.idx)
    }
    pub fn get_multi_nodes(&self, node_ids: &[NodeId]) -> Vec<&Node> {
        node_ids.iter().filter_map(|id| self.get_node(id)).collect()
    }
    pub fn get_node_value<T>(&self, node_id: &NodeId) -> Option<&T>
    where
        T: Any,
    {
        match self.get_node(node_id) {
            Some(Node::Expr(expr)) => (expr as &dyn Any).downcast_ref::<T>(),
            Some(Node::Stmt(stmt)) => (stmt as &dyn Any).downcast_ref::<T>(),
            Some(Node::Part(part)) => (part as &dyn Any).downcast_ref::<T>(),
            None => None,
        }
    }
    pub fn get_multi_node_values<T>(&self, node_ids: &[NodeId]) -> Vec<&T>
    where
        T: Any,
    {
        node_ids
            .iter()
            .filter_map(|id| self.get_node_value::<T>(id))
            .collect()
    }
}
