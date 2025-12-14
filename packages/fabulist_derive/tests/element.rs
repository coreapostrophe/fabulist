use fabulist_core::{
    engine::Progressive,
    error::EngineResult,
    story::{character::Character, resource::Inset},
};
use fabulist_derive::Element;

#[derive(Element, Debug)]
pub struct SampleElement {
    pub text: String,
    pub character: Inset<Character>,
}

impl Progressive for SampleElement {
    type Output = EngineResult<Option<String>>;
    fn next(
        &self,
        _state: &mut fabulist_core::state::State,
        _choice_index: Option<usize>,
    ) -> Self::Output {
        Ok(None)
    }
}

#[derive(Element, Debug)]
pub struct SampleElementUnnamed(pub Inset<Character>);

impl Progressive for SampleElementUnnamed {
    type Output = EngineResult<Option<String>>;
    fn next(
        &self,
        _state: &mut fabulist_core::state::State,
        _choice_index: Option<usize>,
    ) -> Self::Output {
        Ok(None)
    }
}

fn main() {}
