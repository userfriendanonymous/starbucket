
pub use user::*;
pub use project::*;
pub use comment::*;
pub use crawl::*;

pub mod user;
pub mod project;
pub mod comment;
pub mod crawl;

pub trait Query {
    type C;
    fn run(&self, capture: &Self::C) -> bool;
}

#[derive(Debug, Clone)]
pub enum Result<T: Query, E: Query> {
    Ok(T),
    Err(E),
}

impl<T: Query, E: Query> Query for Result<T, E> {
    type C = std::result::Result<T::C, E::C>;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Ok(query) => if let Ok(capture) = capture {
                query.run(capture)
            } else { false },
            Self::Err(query) => if let Err(capture) = capture {
                query.run(capture)
            } else { false },
        }
    }
}

pub type UserResult = Result<Logic<User>, Logic<S2rsError>>;

pub enum CrawlQuery {
    User(Logic<UserResult>),
    // UserProject(LogicQuery<UserProjectQuery>),
    // Project(LogicQuery<ProjectQuery>),
}


#[derive(Debug, Clone)]
pub enum Logic<T: Query> {
    Not(Box<Self>),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    This(T)
}

impl<T: Query> Query for Logic<T> {
    type C = T::C;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::And(left, right) => left.run(capture) && right.run(capture),
            Self::Or(left, right) => left.run(capture) || right.run(capture),
            Self::Not(this) => !this.run(capture),
            Self::This(this) => this.run(capture)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Cmp<T: Ord> {
    Range(std::ops::Range<T>),
    Equals(T),
}

impl<T: Ord> Query for Cmp<T> {
    type C = T;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Range(range) => &range.start >= capture && &range.end < capture,
            Self::Equals(data) => data == capture
        }
    }
}

#[derive(Debug, Clone)]
pub enum Text {
    Contains(String),
    Is(String),
}

impl Query for Text {
    type C = String;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Contains(data) => capture.contains(data),
            Self::Is(data) => capture == data
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bool;

impl Query for Bool {
    type C = bool;
    fn run(&self, capture: &Self::C) -> bool {
        *capture
    }
}
