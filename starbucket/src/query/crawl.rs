use starcrawl::{capture::Capture, location::CrawlLocation};
use super::{Logic, Cmp, Query, User, Project, Option, Result, Text};

pub type S2rsResult<T: Query> = Result<Logic<T>, Logic<S2rsError>>;

#[derive(Debug, Clone)]
pub enum IdWithTitle {
    Id(Logic<Cmp<u64>>),
    Title(Logic<Text>)
}

impl Query for IdWithTitle {
    type C = (u64, String);
    fn run(&self, (id, title): &Self::C) -> bool {
        match self {
            Self::Id(query) => query.run(id),
            Self::Title(query) => query.run(title)
        }
    }
}

#[derive(Debug)]
pub enum S2rsError {
    Status(Logic<Cmp<u16>>),
    Network,
    Parsing
}

impl Query for S2rsError {
    type C = s2rs::api::Error;
    fn run(&self, capture: &Self::C) -> bool {
        type E = s2rs::api::Error;
        match self {
            Self::Status(query) => if let E::Status(status) = capture {
                query.run(&status.as_u16())
            } else { false },
            Self::Network => matches!(capture, E::Network(..)),
            Self::Parsing => matches!(capture, E::Parsing(..)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Chain<T: Query, N: Query> where T::C: Capture, N::C: Clone + Into<CrawlLocation> {
    Next(Logic<Option<Logic<N>>>),
    This(Logic<T>)
}

impl<T: Query, N: Query> Query for Chain<T, N> where T::C: Capture, N::C: Clone + Into<CrawlLocation> {
    type C = starcrawl::capture::Chain<T::C, N::C>;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::This(query) => query.run(&capture.this),
            Self::Next(query) => query.run(&capture.next)
        }
    }
}
