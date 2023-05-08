
use starcrawl::location;
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
pub enum Option<T: Query> {
    Some(T),
    None,
}


#[derive(Debug, Clone)]
pub enum NextDirection {
    Up,
    Down,
}

impl Query for NextDirection {
    type C = location::NextDirection;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Up => matches!(capture, location::NextDirection::Up),
            Self::Down => matches!(capture, location::NextDirection::Down)
        }
    }
}

impl<T: Query> Query for Option<T> {
    type C = std::option::Option<T::C>;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Some(query) => if let Some(data) = capture {
                query.run(data)
            } else { false },
            Self::None => matches!(capture, None)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Vec<T: Query> {
    Any(T)
}

impl<T: Query> Query for Vec<T> {
    type C = std::vec::Vec<T::C>;
    fn run(&self, capture: &Self::C) -> bool {
        match self {
            Self::Any(query) => {
                let mut caught = false;
                for item in capture.iter() {
                    if query.run(item) {
                        caught = true;
                        break;
                    }
                }
                caught
            }
        }
    }
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
            Self::Contains(data) => capture.to_ascii_lowercase().contains(&data.to_ascii_lowercase()),
            Self::Is(data) => capture.to_ascii_lowercase() == data.to_ascii_lowercase()
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
