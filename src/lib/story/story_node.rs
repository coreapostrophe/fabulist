use self::{dialogue::Dialogue, part::Part};

pub mod dialogue;
pub mod part;

pub enum StoryNode {
    Part(Part),
    Dialogue(Dialogue),
}
