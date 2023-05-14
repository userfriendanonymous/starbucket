use super::{Logic, Text, Cmp, Query, S2rsError, Result, S2rsResult};

pub type ProjectResult = S2rsResult<Project>;

#[derive(Debug, Clone)]
pub enum Project {
    Description(Logic<Text>),
    Instructions(Logic<Text>),
    Stats(Logic<ProjectStats>),
    Text(Logic<Text>),
}

impl Query for Project {
    type C = s2rs::api::Project;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Description(query) => query.run(&capture.description),
            Self::Instructions(query) => query.run(&capture.instructions),
            Self::Stats(query) => query.run(&capture.stats),
            Self::Text(query) => query.run(&capture.description) || query.run(&capture.instructions) || query.run(&capture.title)
        }
    }
}

#[derive(Debug, Clone)]
pub enum ProjectStats {
    Loves(Logic<Cmp<u64>>),
    Favorites(Logic<Cmp<u64>>),
    Remixes(Logic<Cmp<u64>>),
    Views(Logic<Cmp<u64>>),
}

impl Query for ProjectStats {
    type C = s2rs::api::ProjectStats;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Loves(query) => query.run(&capture.loves),
            Self::Favorites(query) => query.run(&capture.favorites),
            Self::Remixes(query) => query.run(&capture.remixes),
            Self::Views(query) => query.run(&capture.views),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Project3 {
    Title(Logic<Text>),
    Description(Logic<Text>),
    Instructions(Logic<Text>),
    Text(Logic<Text>)
}

impl Query for Project3 {
    type C = s2rs::api::Project3;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Title(query) => query.run(&capture.title),
            Self::Description(query) => query.run(&capture.description),
            Self::Instructions(query) => query.run(&capture.instructions),
            Self::Text(query) => query.run(&capture.title) || query.run(&capture.description) || query.run(&capture.instructions)
        }
    }
}
