use starcrawl::{capture::Capture, location::CrawlLocation};
use super::{Logic, Cmp, Query, User, Project, Option, Result, Text, LogicR, CmpR};

pub type S2rsResult<T: Query> = Result<Logic<T>, Logic<S2rsError>>;

#[derive(Debug)]
pub enum S2rsError {
    Status(Logic<Cmp<u16>>),
    Network,
    Parsing
}

#[derive(Debug, Clone)]
pub enum S2rsErrorR {
    Network,
    Parsing,
    Status(LogicR<CmpR>)
}

impl Query for S2rsError {
    type R = S2rsErrorR;
    type C = s2rs::api::Error;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        type E = s2rs::api::Error;
        match self {
            Self::Status(query) => if let E::Status(status) = capture {
                Some(Self::R::Status(query.run(&status.as_u16())?))
            } else { None },
            Self::Network => if let E::Network(..) = capture { Some(Self::R::Network) } else { None },
            Self::Parsing => if let E::Parsing(..) = capture { Some(Self::R::Parsing) } else { None },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Chain<T: Query, N: Query> where T::C: Capture, N::C: Clone + Into<CrawlLocation> {
    Next(Logic<Option<Logic<N>>>),
    This(Logic<T>)
}

#[derive(Debug, Clone)]
pub enum ChainR<T, N> {
    Next(LogicR<std::option::Option<LogicR<N>>>),
    This(LogicR<T>)
}

impl<T: Query, N: Query> Query for Chain<T, N> where T::C: Capture, N::C: Clone + Into<CrawlLocation> {
    type R = ChainR<T::R, N::R>;
    type C = starcrawl::capture::Chain<T::C, N::C>;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::This(query) => Some(Self::R::This(query.run(&capture.this)?)),
            Self::Next(query) => Some(Self::R::Next(query.run(&capture.next)?))
        }
    }
}
