use std::collections::BTreeMap;

use super::{Block, Expr};

pub type FunctionId = usize;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StoryProgram {
    pub start_part: String,
    pub metadata: BTreeMap<String, Expr>,
    pub parts: Vec<PartSpec>,
    pub functions: Vec<FunctionSpec>,
}

impl StoryProgram {
    pub fn find_part_index(&self, part_id: &str) -> Option<usize> {
        self.parts.iter().position(|part| part.id == part_id)
    }

    pub fn function(&self, function_id: FunctionId) -> Option<&FunctionSpec> {
        self.functions.get(function_id)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PartSpec {
    pub id: String,
    pub steps: Vec<StepSpec>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum StepSpec {
    Narration(QuoteSpec),
    Dialogue(DialogueSpec),
    Selection(SelectionSpec),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DialogueSpec {
    pub speaker: String,
    pub quote: QuoteSpec,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SelectionSpec {
    pub choices: Vec<QuoteSpec>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QuoteSpec {
    pub node_id: usize,
    pub text: String,
    pub properties: BTreeMap<String, Expr>,
    pub next_action: Option<FunctionId>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FunctionSpec {
    pub id: FunctionId,
    pub node_id: usize,
    pub params: Vec<String>,
    pub body: Block,
}
