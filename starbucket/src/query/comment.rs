use super::{Query, Logic, Cmp, Text, Bool};

#[derive(Debug, Clone)]
pub enum Comment {
    Id(Logic<Cmp<u64>>),
    Content(Logic<Text>),
    Author(Logic<CommentAuthor>)

}

impl Query for Comment {
    type C = s2rs::api::Comment;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Content(query) => query.run(&capture.content),
            Self::Id(query) => query.run(&capture.id),
            Self::Author(query) => query.run(&capture.author),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CommentAuthor {
    Name(Logic<Text>),
    Id(Logic<Cmp<u64>>),
    ScratchTeam(Logic<Bool>)
}

impl Query for CommentAuthor {
    type C = s2rs::api::CommentAuthor;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Name(query) => query.run(&capture.name),
            Self::Id(query) => query.run(&capture.id),
            Self::ScratchTeam(query) => query.run(&capture.scratch_team)
        }
    }
}