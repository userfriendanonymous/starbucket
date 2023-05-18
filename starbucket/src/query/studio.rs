use starcrawl::location;

use super::{Query, Logic, Cmp, Text, S2rsResult, IdWithTitle, NextDirection, Chain, Option, Comment, Vec, S2rsError, Result, LogicR, CmpR, TextR, S2rsErrorR};

pub type StudioResult = S2rsResult<Studio>;
pub type StudioActivityResult = Result<Logic<Chain<Vec<Logic<StudioAction>>, StudioActivityLocation>>, Logic<StudioActivityError>>;
pub type StudioCommentsResult = S2rsResult<Chain<Vec<Logic<Comment>>, StudioCommentsLocation>>;
pub type StudioProjectsResult = S2rsResult<Chain<Vec<Logic<StudioProject>>, StudioProjectsLocation>>;

// region: locations
#[derive(Debug, Clone)]
pub enum StudioLocationR {
    Page(LogicR<CmpR>),
    Id(LogicR<CmpR>),
    Next(std::option::Option<location::NextDirection>),
}

#[derive(Debug, Clone)]
pub enum StudioActivityLocation {
    Page(Logic<Cmp<usize>>),
    Id(Logic<Cmp<u64>>),
    Next(Option<NextDirection>),
}

impl Query for StudioActivityLocation {
    type R = StudioLocationR;
    type C = location::StudioActivity;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::Page(query) => Some(Self::R::Page(query.run(&capture.page)?)),
            Self::Id(query) => Some(Self::R::Id(query.run(&capture.id)?)),
            Self::Next(query) => Some(Self::R::Next(query.run(&capture.next)?))
        }
    }
}


#[derive(Debug, Clone)]
pub enum StudioCommentsLocation {
    Page(Logic<Cmp<usize>>),
    Id(Logic<Cmp<u64>>),
    Next(Option<NextDirection>),
}

impl Query for StudioCommentsLocation {
    type R = StudioLocationR;
    type C = location::StudioComments;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::Page(query) => Some(Self::R::Page(query.run(&capture.page)?)),
            Self::Id(query) => Some(Self::R::Id(query.run(&capture.id)?)),
            Self::Next(query) => Some(Self::R::Next(query.run(&capture.next)?))
        }
    }
}

#[derive(Debug, Clone)]
pub enum StudioProjectsLocation {
    Page(Logic<Cmp<usize>>),
    Id(Logic<Cmp<u64>>),
    Next(Option<NextDirection>),
}

impl Query for StudioProjectsLocation {
    type C = location::StudioProjects;
    type R = StudioLocationR;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::Page(query) => Some(Self::R::Page(query.run(&capture.page)?)),
            Self::Id(query) => Some(Self::R::Id(query.run(&capture.id)?)),
            Self::Next(query) => Some(Self::R::Next(query.run(&capture.next)?))
        }
    }
}
// endregion: locations

#[derive(Debug, Clone)]
pub enum Studio {
    Title(Logic<Text>),
    Description(Logic<Text>),
    Id(Logic<Cmp<u64>>),
    Host(Logic<Cmp<u64>>),
    Stats(Logic<StudioStats>),
}

#[derive(Debug, Clone)]
pub enum StudioR {
    Text(LogicR<TextR>),
    Cmp(LogicR<CmpR>),
    Stats(LogicR<StudioStatsR>),
}

impl Query for Studio {
    type C = s2rs::api::Studio;
    type R = StudioR;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::Stats(query) => Some(Self::R::Stats(query.run(&capture.stats)?)),
            Self::Title(query) => Some(Self::R::Text(query.run(&capture.title)?)),
            Self::Description(query) => Some(Self::R::Text(query.run(&capture.description)?)),
            Self::Id(query) => Some(Self::R::Cmp(query.run(&capture.id)?)),
            Self::Host(query) => Some(Self::R::Cmp(query.run(&capture.host)?)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum StudioStats {
    Comments(Logic<Cmp<u64>>),
    Followers(Logic<Cmp<u64>>),
    Managers(Logic<Cmp<u64>>),
    Projects(Logic<Cmp<u64>>),
}

#[derive(Debug, Clone)]
pub enum StudioStatsR {
    Cmp(LogicR<CmpR>),
}

impl Query for StudioStats {
    type C = s2rs::api::StudioStats;
    type R = StudioStatsR;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::Comments(query) => Some(Self::R::Cmp(query.run(&capture.comments)?)),
            Self::Followers(query) => Some(Self::R::Cmp(query.run(&capture.followers)?)),
            Self::Managers(query) => Some(Self::R::Cmp(query.run(&capture.managers)?)),
            Self::Projects(query) => Some(Self::R::Cmp(query.run(&capture.projects)?)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum StudioAction {
    ActorName(Logic<Text>),
    ActorId(Logic<Cmp<u64>>),
    Id(Logic<Cmp<u64>>),
    Event(Logic<StudioActionEvent>)
}

#[derive(Debug, Clone)]
pub enum StudioActionR {
    Text(LogicR<TextR>),
    Cmp(LogicR<CmpR>),
    Event(LogicR<StudioActionEventR>)
}

impl Query for StudioAction {
    type R = StudioActionR;
    type C = s2rs::api::StudioAction;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::ActorId(query) => Some(Self::R::Cmp(query.run(&capture.actor_id)?)),
            Self::ActorName(query) => Some(Self::R::Text(query.run(&capture.actor_name)?)),
            Self::Id(query) => Some(Self::R::Cmp(query.run(&capture.id)?)),
            Self::Event(query) => Some(Self::R::Event(query.run(&capture.event)?)),
        }
    }
}


#[derive(Debug, Clone)]
pub enum StudioActionEvent {
    AcceptInvite(Logic<Text>),
    AddProject(Logic<IdWithTitle>),
    Promote(Logic<Text>),
    RemoveProject(Logic<IdWithTitle>),
    Update
}

#[derive(Debug, Clone)]
pub enum StudioActionEventR {
    Text(LogicR<TextR>),
    IdWithTitle(LogicR<IdWithTitleR>),
    None
}

impl Query for StudioActionEvent {
    type R = StudioActionEventR;
    type C = s2rs::api::StudioActionEvent;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        type C = s2rs::api::StudioActionEvent;
        match self {
            Self::AcceptInvite(query) => if let C::AcceptInvite { from_name } = capture {
                Some(Self::R::Text(query.run(from_name)?))
            } else { None },
            Self::AddProject(query) => if let C::AddProject { id, title } = capture {
                Some(Self::R::IdWithTitle(query.run(&(*id, title.clone()))?))
            } else { None },
            Self::Promote(query) => if let C::Promote { name } = capture {
                Some(Self::R::IdWithTitle(query.run(&name)?))
            } else { None },
            Self::RemoveProject(query) => if let C::RemoveProject { id, title } = capture {
                Some(Self::R::IdWithTitle(query.run(&(*id, title.clone()))?))
            } else { None },
            _ => todo!()
        }
    }
}

#[derive(Debug)]
pub enum StudioActivityError {
    Parsing(),
    This(S2rsError)
}

#[derive(Debug, Clone)]
pub enum StudioActivityErrorR {
    None,
    This(S2rsErrorR)
}


impl Query for StudioActivityError {
    type R = StudioActivityErrorR;
    type C = s2rs::api::GetStudioActivityError;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        type C = s2rs::api::GetStudioActivityError;
        match self {
            Self::Parsing() => if let C::Parsing(..) = capture { Some(Self::R::None) } else { None },
            Self::This(query) => if let C::This(e) = capture {
                Some(Self::R::This(query.run(e)?))
            } else { None }
        }
    }
}

#[derive(Debug, Clone)]
pub enum StudioProject {
    Id(Logic<Cmp<u64>>),
    Title(Logic<Text>),
    AuthorName(Logic<Text>),
    ActorId(Logic<Cmp<u64>>),
    CreatorId(Logic<Cmp<u64>>)
}

#[derive(Debug, Clone)]
pub enum StudioProjectR {
    Cmp(LogicR<CmpR>),
    Text(LogicR<TextR>),
}

impl Query for StudioProject {
    type R = StudioProjectR;
    type C = s2rs::api::StudioProject;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::ActorId(query) => Some(Self::R::Cmp(query.run(&capture.actor_id)?)),
            Self::AuthorName(query) => Some(Self::R::Text(query.run(&capture.name)?)),
            Self::CreatorId(query) => Some(Self::R::Cmp(query.run(&capture.creator_id)?)),
            Self::Id(query) => Some(Self::R::Cmp(query.run(&capture.id)?)),
            Self::Title(query) => Some(Self::R::Text(query.run(&capture.title)?))
        }
    }
}
