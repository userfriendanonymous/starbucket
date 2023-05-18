use s2rs::Emoji;
use starcrawl::{location::{CrawlLocation, self}, capture::Capture};
use super::{Logic, Text, Query, Cmp, Bool, Result, S2rsError, Project3, Option, Chain, NextDirection, Vec, S2rsResult, Comment, CmpR, TextR, LogicR, VecR, S2rsErrorR};

pub type UserResult = Result<Logic<User>, Logic<S2rsError>>;
pub type UserProjectsResult = S2rsResult<Chain<Vec<Logic<Project3>>, UserProjectsLocation>>;
pub type UserCommentsResult = Result<Logic<Chain<Vec<Logic<UserComment>>, UserCommentsLocation>>, Logic<UserCommentsError>>;
pub type UserFollowersResult = S2rsResult<Chain<Vec<Logic<User>>, UserFollowersLocation>>;
pub type UserFollowingResult = S2rsResult<Chain<Vec<Logic<User>>, UserFollowingLocation>>;
pub type UserProjectCommentsResult = S2rsResult<Chain<Vec<Comment>, UserProjectCommentsLocation>>;

// region: Locations
#[derive(Debug, Clone)]
pub enum UserChainR {
    Cmp(LogicR<CmpR>),
    Text(LogicR<TextR>),
    Next(std::option::Option<location::NextDirection>)
}

#[derive(Debug, Clone)]
pub enum UserProjectCommentsLocation {
    Page(Logic<Cmp<usize>>),
    Name(Logic<Text>),
    Next(Option<NextDirection>),
}

impl Query for UserProjectCommentsLocation {
    type R = UserChainR;
    type C = location::UserProjectComments;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::Page(query) => Some(Self::R::Cmp(query.run(&capture.page)?)),
            Self::Name(query) => Some(Self::R::Text(query.run(&capture.name.clone().into())?)),
            Self::Next(query) => Some(Self::R::Next(query.run(&capture.next)?))
        }
    }
}

#[derive(Debug, Clone)]
pub enum UserCommentsLocation {
    Page(Logic<Cmp<u8>>),
    Name(Logic<Text>),
    Next(Option<NextDirection>),
}

impl Query for UserCommentsLocation {
    type R = UserChainR;
    type C = location::UserComments;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::Page(query) => Some(Self::R::Cmp(query.run(&capture.page)?)),
            Self::Name(query) => Some(Self::R::Text(query.run(&capture.name.clone().into())?)),
            Self::Next(query) => Some(Self::R::Next(query.run(&capture.next)?))
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
    type R = UserChainR;
    type C = location::UserProjects;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::Page(query) => Some(Self::R::Cmp(query.run(&capture.page)?)),
            Self::Name(query) => Some(Self::R::Text(query.run(&capture.name.clone().into())?)),
            Self::Next(query) => Some(Self::R::Next(query.run(&capture.next)?))
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
    type R = UserChainR;
    type C = location::UserFollowers;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::Page(query) => Some(Self::R::Cmp(query.run(&capture.page)?)),
            Self::Name(query) => Some(Self::R::Text(query.run(&capture.name.clone().into())?)),
            Self::Next(query) => Some(Self::R::Next(query.run(&capture.next)?))
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
    type R = UserChainR;
    type C = location::UserFollowing;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::Page(query) => Some(Self::R::Cmp(query.run(&capture.page)?)),
            Self::Name(query) => Some(Self::R::Text(query.run(&capture.name.clone().into())?)),
            Self::Next(query) => Some(Self::R::Next(query.run(&capture.next)?))
        }
    }
}
// endregion: Locations

#[derive(Debug)]
pub enum User {
    Id(Logic<Cmp<u64>>),
    Profile(UserProfile),
    ScratchTeam(Logic<Bool>),
}

#[derive(Debug, Clone)]
pub enum UserR {
    Cmp(LogicR<CmpR>),
    Profile(UserProfileR),
    Logic(LogicR<()>),
}

impl Query for User {
    type R = UserR;
    type C = s2rs::api::User;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::Id(query) => Some(Self::R::Cmp(query.run(&capture.id)?)),
            Self::Profile(query) => Some(Self::R::Profile(query.run(&capture.profile)?)),
            Self::ScratchTeam(query) => Some(Self::R::Logic(query.run(&capture.scratch_team)?)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum UserProfile {
    Bio(Logic<Text>),
    Wiwo(Logic<Text>),
    Country(Logic<Text>),
    Id(Logic<Cmp<u64>>),
}

#[derive(Debug, Clone)]
pub enum UserProfileR {
    Text(LogicR<TextR>),
    Cmp(LogicR<CmpR>)
}

impl Query for UserProfile {
    type R = UserProfileR;
    type C = s2rs::api::UserProfile;
    fn run(&self, capture: &Self::C) -> std::option::Option<UserProfileR> {
        match self {
            Self::Bio(query) => Some(Self::R::Text(query.run(&capture.bio)?)),
            Self::Wiwo(query) => Some(Self::R::Text(query.run(&capture.status)?)),
            Self::Country(query) => Some(Self::R::Text(query.run(&capture.country)?)),
            Self::Id(query) => Some(Self::R::Cmp(query.run(&capture.id)?)),
        }
    }
}


#[derive(Debug)]
pub enum UserCommentsError {
    Parsing,
    This(S2rsError)
}

#[derive(Debug, Clone)]
pub enum UserCommentsErrorR {
    Parsing,
    This(S2rsErrorR)
}

impl Query for UserCommentsError {
    type R = UserCommentsErrorR;
    type C = s2rs::api::GetUserCommentsError;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        type E = s2rs::api::GetUserCommentsError;
        match self {
            Self::Parsing => if let E::Parsing = capture { Some(Self::R::Parsing) } else { None },
            Self::This(query) => if let E::This(this) = capture {
                Some(Self::R::This(query.run(this)?))
            } else { None }
        }
    }
}

#[derive(Debug, Clone)]
pub enum UserComment {
    AuthorId(Logic<Cmp<u64>>),
    AuthorName(Logic<Text>),
    Content(Logic<Vec<UserCommentContentFragment>>)
}

#[derive(Debug, Clone)]
pub enum UserCommentR {
    Text(LogicR<TextR>),
    Cmp(LogicR<CmpR>),
    Content(LogicR<VecR<UserCommentContentFragmentR>>)
}

impl Query for UserComment {
    type R = UserCommentR;
    type C = s2rs::api::UserComment;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::AuthorName(query) => Some(Self::R::Text(query.run(&capture.author_name)?)),
            Self::AuthorId(query) => Some(Self::R::Cmp(query.run(&capture.author_id)?)),
            Self::Content(query) => Some(Self::R::Content(query.run(&capture.content)?))
        }
    }
}

impl Query for Emoji {
    type R = ();
    type C = Self;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        if capture == self { Some(()) } else { None }
    }
}

#[derive(Debug, Clone)]
pub enum UserCommentContentFragment {
    Emoji(Logic<Emoji>),
    Tag(Logic<Text>)
}

#[derive(Debug, Clone)]
pub enum UserCommentContentFragmentR {
    Logic(LogicR<()>),
    Text(LogicR<TextR>),
}

impl Query for UserCommentContentFragment {
    type R = UserCommentContentFragmentR;
    type C = s2rs::api::UserCommentContentFragment;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::Emoji(query) => if let Self::C::Emoji(emoji) = capture {
                Some(Self::R::Logic(query.run(emoji)?))
            } else { None },
            Self::Tag(query) => if let Self::C::Tag(tag) = capture {
                Some(Self::R::Text(query.run(tag)?))
            } else { None }
        }
    }
}