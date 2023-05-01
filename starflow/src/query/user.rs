use crate::entities;
use super::{Logic, Text, Query, Cmp, Bool};


pub enum UserProject {
    Title(Logic<Text>),
    Author(Logic<Text>),
    Description(Logic<Text>),
    Instructions(Logic<Text>),
}

// impl Query for UserProjectQuery {
//     type C = UserProjectCapture;
//     fn run(&self, capture: &Self::C) -> bool {
//         match self {
//             Self::Author(query) => query.run(capture.)
//         }
//     }
// }

#[derive(Debug)]
pub enum UserMeta {
    Id(Logic<Cmp<u64>>),
    Profile(UserProfile),
    ScratchTeam(Logic<Bool>)
}

impl Query for UserMeta {
    type C = entities::UserMeta;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Id(query) => query.run(&capture.id),
            Self::Profile(query) => query.run(&capture.profile),
            Self::ScratchTeam(query) => query.run(&capture.scratch_team)
        }
    }
}

#[derive(Debug, Clone)]
pub enum UserProfile {
    Bio(Logic<Text>),
    Wiwo(Logic<Text>),
    Country(Logic<Text>),
    Id(Logic<Cmp<u64>>)
}

impl Query for UserProfile {
    type C = entities::UserProfile;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Bio(query) => query.run(&capture.bio),
            Self::Wiwo(query) => query.run(&capture.wiwo),
            Self::Country(query) => query.run(&capture.country),
            Self::Id(query) => query.run(&capture.id)
        }
    }
}