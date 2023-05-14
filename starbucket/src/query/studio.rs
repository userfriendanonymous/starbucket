use starcrawl::location;

use super::{Query, Logic, Cmp, Text, S2rsResult, IdWithTitle, NextDirection, Chain, Option, Comment, Vec, S2rsError, Result};

pub type StudioResult = S2rsResult<Studio>;
pub type StudioActivityResult = Result<Logic<Chain<Vec<Logic<StudioAction>>, StudioActivityLocation>>, Logic<StudioActivityError>>;
pub type StudioCommentsResult = S2rsResult<Chain<Vec<Logic<Comment>>, StudioCommentsLocation>>;
pub type StudioProjectsResult = S2rsResult<Chain<Vec<Logic<StudioProject>>, StudioProjectsLocation>>;

// region: locations
#[derive(Debug, Clone)]
pub enum StudioActivityLocation {
    Page(Logic<Cmp<usize>>),
    Id(Logic<Cmp<u64>>),
    Next(Option<NextDirection>),
}

impl Query for StudioActivityLocation {
    type C = location::StudioActivity;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Page(query) => query.run(&capture.page),
            Self::Id(query) => query.run(&capture.id),
            Self::Next(query) => query.run(&capture.next)
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
    type C = location::StudioComments;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Page(query) => query.run(&capture.page),
            Self::Id(query) => query.run(&capture.id),
            Self::Next(query) => query.run(&capture.next)
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
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Page(query) => query.run(&capture.page),
            Self::Id(query) => query.run(&capture.id),
            Self::Next(query) => query.run(&capture.next)
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
    Text(Logic<Text>)
}

impl Query for Studio {
    type C = s2rs::api::Studio;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Stats(query) => query.run(&capture.stats),
            Self::Title(query) => query.run(&capture.title),
            Self::Description(query) => query.run(&capture.description),
            Self::Id(query) => query.run(&capture.id),
            Self::Host(query) => query.run(&capture.host),
            Self::Text(query) => query.run(&capture.description) || query.run(&capture.title)
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

impl Query for StudioStats {
    type C = s2rs::api::StudioStats;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Comments(query) => query.run(&capture.comments),
            Self::Followers(query) => query.run(&capture.followers),
            Self::Managers(query) => query.run(&capture.managers),
            Self::Projects(query) => query.run(&capture.projects),
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

impl Query for StudioAction {
    type C = s2rs::api::StudioAction;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::ActorId(query) => query.run(&capture.actor_id),
            Self::ActorName(query) => query.run(&capture.actor_name),
            Self::Id(query) => query.run(&capture.id),
            Self::Event(query) => query.run(&capture.event),
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

impl Query for StudioActionEvent {
    type C = s2rs::api::StudioActionEvent;
    fn run(&self, capture: &Self::C) -> bool {
        type C = s2rs::api::StudioActionEvent;
        match self {
            Self::AcceptInvite(query) => if let C::AcceptInvite { from_name } = capture {
                query.run(from_name)
            } else { false },
            Self::AddProject(query) => if let C::AddProject { id, title } = capture {
                query.run(&(*id, title.clone()))
            } else { false },
            Self::Promote(query) => if let C::Promote { name } = capture {
                query.run(&name)
            } else { false },
            Self::RemoveProject(query) => if let C::RemoveProject { id, title } = capture {
                query.run(&(*id, title.clone()))
            } else { false },
            _ => todo!()
        }
    }
}

#[derive(Debug)]
pub enum StudioActivityError {
    Parsing(),
    This(S2rsError)
}


impl Query for StudioActivityError {
    type C = s2rs::api::GetStudioActivityError;
    fn run(&self, capture: &Self::C) -> bool {
        type C = s2rs::api::GetStudioActivityError;
        match self {
            Self::Parsing() => matches!(C::Parsing, capture),
            Self::This(query) => if let C::This(e) = capture {
                query.run(e)
            } else { false }
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

impl Query for StudioProject {
    type C = s2rs::api::StudioProject;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::ActorId(query) => query.run(&capture.actor_id),
            Self::AuthorName(query) => query.run(&capture.name),
            Self::CreatorId(query) => query.run(&capture.creator_id),
            Self::Id(query) => query.run(&capture.id),
            Self::Title(query) => query.run(&capture.title)
        }
    }
}
