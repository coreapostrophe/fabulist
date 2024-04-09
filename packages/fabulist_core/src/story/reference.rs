use std::{collections::VecDeque, fmt::Display};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct ListKey<T>(pub VecDeque<T>);

impl<T, const N: usize> From<[T; N]> for ListKey<T> {
    fn from(value: [T; N]) -> Self {
        Self(VecDeque::from(value))
    }
}

impl<const N: usize> From<[&str; N]> for ListKey<String> {
    fn from(value: [&str; N]) -> Self {
        let value: Vec<String> = value.iter().map(|s| s.to_string()).collect();
        Self(VecDeque::from(value))
    }
}

impl<const N: usize> From<[&String; N]> for ListKey<String> {
    fn from(value: [&String; N]) -> Self {
        let value: Vec<String> = value.iter().map(|s| s.to_string()).collect();
        Self(VecDeque::from(value))
    }
}

impl<T> From<Vec<T>> for ListKey<T> {
    fn from(value: Vec<T>) -> Self {
        Self(VecDeque::from(value))
    }
}

impl From<Vec<&str>> for ListKey<String> {
    fn from(value: Vec<&str>) -> Self {
        let value: Vec<String> = value.iter().map(|s| s.to_string()).collect();
        Self(VecDeque::from(value))
    }
}

impl From<&str> for ListKey<String> {
    fn from(value: &str) -> Self {
        Self(VecDeque::from([value.to_owned()]))
    }
}

impl From<String> for ListKey<String> {
    fn from(value: String) -> Self {
        Self(VecDeque::from([value]))
    }
}

impl From<&String> for ListKey<String> {
    fn from(value: &String) -> Self {
        Self(VecDeque::from([value.to_owned()]))
    }
}

impl Display for ListKey<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct DialogueIndex {
    pub part_key: ListKey<String>,
    pub dialogue_index: usize,
}
