
use super::{Logic, Text, Cmp, Query, S2rsResult, LogicR, TextR, CmpR};

pub type ProjectResult = S2rsResult<Project>;

#[derive(Debug, Clone)]
pub enum Project {
    Description(Logic<Text>),
    Instructions(Logic<Text>),
    Stats(Logic<ProjectStats>),
}

#[derive(Debug, Clone)]
pub enum ProjectR {
    Text(LogicR<TextR>),
    Stats(LogicR<ProjectStatsR>),
}

impl Query for Project {
    type R = ProjectR;
    type C = s2rs::api::Project;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::Description(query) => Some(Self::R::Text(query.run(&capture.description)?)),
            Self::Instructions(query) => Some(Self::R::Text(query.run(&capture.instructions)?)),
            Self::Stats(query) => Some(Self::R::Stats(query.run(&capture.stats)?)),
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

#[derive(Debug, Clone)]
pub enum ProjectStatsR {
    Cmp(LogicR<CmpR>),
}

impl Query for ProjectStats {
    type R = ProjectStatsR;
    type C = s2rs::api::ProjectStats;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::Loves(query) => Some(Self::R::Cmp(query.run(&capture.loves)?)),
            Self::Favorites(query) => Some(Self::R::Cmp(query.run(&capture.favorites)?)),
            Self::Remixes(query) => Some(Self::R::Cmp(query.run(&capture.remixes)?)),
            Self::Views(query) => Some(Self::R::Cmp(query.run(&capture.views)?)),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Project3 {
    Title(Logic<Text>),
    Description(Logic<Text>),
    Instructions(Logic<Text>),
}

#[derive(Clone, Debug)]
pub enum Project3R {
    Text(LogicR<TextR>),
}

impl Query for Project3 {
    type R = Project3R;
    type C = s2rs::api::Project3;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::Title(query) => Some(Self::R::Text(query.run(&capture.title)?)),
            Self::Description(query) => Some(Self::R::Text(query.run(&capture.description)?)),
            Self::Instructions(query) => Some(Self::R::Text(query.run(&capture.instructions)?)),
        }
    }
}
