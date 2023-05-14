use starcrawl::{location::{CrawlLocation, self}, capture::Capture};

use super::{Logic, Text, Query, Cmp, Bool, Result, S2rsError, Project3, Option, Chain, NextDirection, Vec, S2rsResult};

pub type UserResult = Result<Logic<User>, Logic<S2rsError>>;
pub type UserProjectsResult = S2rsResult<Chain<Vec<Logic<Project3>>, UserProjectsLocation>>;
pub type UserCommentsResult = Result<Logic<Chain<Vec<Logic<UserComment>>, UserCommentsLocation>>, Logic<UserCommentsError>>;
pub type UserFollowersResult = S2rsResult<Chain<Vec<Logic<User>>, UserFollowersLocation>>;
pub type UserFollowingResult = S2rsResult<Chain<Vec<Logic<User>>, UserFollowingLocation>>;

// region: Locations
#[derive(Debug, Clone)]
pub enum UserCommentsLocation {
    Page(Logic<Cmp<u8>>),
    Name(Logic<Text>),
    Next(Option<NextDirection>),
}

impl Query for UserCommentsLocation {
    type C = location::UserComments;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Page(query) => query.run(&capture.page),
            Self::Name(query) => query.run(&capture.name.clone().into()),
            Self::Next(query) => query.run(&capture.next)
        }
    }
}


#[derive(Debug, Clone)]
pub enum UserProjectsLocation {
    Page(Logic<Cmp<usize>>),
    Name(Logic<Text>),
    Next(Option<NextDirection>),
}

impl Query for UserProjectsLocation {
    type C = location::UserProjects;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Page(query) => query.run(&capture.page),
            Self::Name(query) => query.run(&capture.name.clone().into()),
            Self::Next(query) => query.run(&capture.next)
        }
    }
}

#[derive(Debug, Clone)]
pub enum UserFollowersLocation {
    Page(Logic<Cmp<usize>>),
    Name(Logic<Text>),
    Next(Option<NextDirection>),
}

impl Query for UserFollowersLocation {
    type C = location::UserFollowers;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Page(query) => query.run(&capture.page),
            Self::Name(query) => query.run(&capture.name.clone().into()),
            Self::Next(query) => query.run(&capture.next)
        }
    }
}

#[derive(Debug, Clone)]
pub enum UserFollowingLocation {
    Page(Logic<Cmp<usize>>),
    Name(Logic<Text>),
    Next(Option<NextDirection>),
}

impl Query for UserFollowingLocation {
    type C = location::UserFollowing;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Page(query) => query.run(&capture.page),
            Self::Name(query) => query.run(&capture.name.clone().into()),
            Self::Next(query) => query.run(&capture.next)
        }
    }
}
// endregion: Locations

#[derive(Debug)]
pub enum User {
    Id(Logic<Cmp<u64>>),
    Profile(UserProfile),
    ScratchTeam(Logic<Bool>),
    Text(Logic<Text>)
}

impl Query for User {
    type C = s2rs::api::User;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Id(query) => query.run(&capture.id),
            Self::Profile(query) => query.run(&capture.profile),
            Self::ScratchTeam(query) => query.run(&capture.scratch_team),
            Self::Text(query) => UserProfile::Text(query.clone()).run(&capture.profile)
        }
    }
}

#[derive(Debug, Clone)]
pub enum UserProfile {
    Bio(Logic<Text>),
    Wiwo(Logic<Text>),
    Country(Logic<Text>),
    Id(Logic<Cmp<u64>>),
    Text(Logic<Text>)
}


impl Query for UserProfile {
    type C = s2rs::api::UserProfile;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Bio(query) => query.run(&capture.bio),
            Self::Wiwo(query) => query.run(&capture.status),
            Self::Country(query) => query.run(&capture.country),
            Self::Id(query) => query.run(&capture.id),
            Self::Text(query) => query.run(&capture.bio) || query.run(&capture.status)
        }
    }
}


#[derive(Debug)]
pub enum UserCommentsError {
    Parsing,
    This(S2rsError)
}

impl Query for UserCommentsError {
    type C = s2rs::api::GetUserCommentsError;
    fn run(&self, capture: &Self::C) -> bool {
        type E = s2rs::api::GetUserCommentsError;
        match self {
            Self::Parsing => matches!(capture, E::Parsing),
            Self::This(query) => if let E::This(this) = capture {
                query.run(this)
            } else { false }
        }
    }
}

#[derive(Debug, Clone)]
pub enum UserComment {
    AuthorId(Logic<Cmp<u64>>),
    AuthorName(Logic<Text>),
    Content()
}

impl Query for UserComment {
    type C = s2rs::api::UserComment;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::AuthorName(query) => query.run(&capture.author_name),
            Self::AuthorId(query) => query.run(&capture.author_id),
            Self::Content() => false
        }
    }
}
