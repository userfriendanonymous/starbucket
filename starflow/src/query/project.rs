use crate::entities;
use super::{Logic, Text, Cmp, Query, Result, S2rsError};

pub type ProjectMetaResult = Result<ProjectMeta, S2rsError>;

pub enum ProjectMeta {
    Description(Logic<Text>),
    Instructions(Logic<Text>),
    Stats(Logic<ProjectStats>),
}

impl Query for ProjectMeta {
    type C = entities::ProjectMeta;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Description(query) => query.run(&capture.description),
            Self::Instructions(query) => query.run(&capture.instructions),
            Self::Stats(query) => query.run(&capture.stats)
        }
    }
}

pub enum ProjectStats {
    Loves(Logic<Cmp<u32>>),
    Favorites(Logic<Cmp<u32>>),
    Remixes(Logic<Cmp<u32>>),
    Views(Logic<Cmp<u32>>),
}

impl Query for ProjectStats {
    type C = entities::ProjectStats;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Loves(query) => query.run(&capture.loves),
            Self::Favorites(query) => query.run(&capture.favorites),
            Self::Remixes(query) => query.run(&capture.remixes),
            Self::Views(query) => query.run(&capture.views),
        }
    }
}