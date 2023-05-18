
use starcrawl::location;
pub use user::*;
pub use project::*;
pub use comment::*;
pub use crawl::*;
pub use studio::*;

pub mod user;
pub mod project;
pub mod comment;
pub mod crawl;
pub mod studio;

pub trait Query {
    type C;
    type R;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R>;
}

#[derive(Debug, Clone)]
pub enum Option<T: Query> {
    Some(T),
    None,
}

impl<T: Query> Query for Option<T> {
    type R = std::option::Option<T::R>;
    type C = std::option::Option<T::C>;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        Some(match self {
            Self::Some(query) => if let Some(data) = capture {
                Some(Self::R::Some(query.run(data)?))
            } else { None },
            Self::None => if let None = capture { Some(Self::R::None) } else { None }
        }?)
    }
}


#[derive(Debug, Clone)]
pub enum NextDirection {
    Up,
    Down,
}

impl Query for NextDirection {
    type R = location::NextDirection;
    type C = location::NextDirection;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::Up => if let location::NextDirection::Up = capture {
                Some(Self::R::Up)
            } else { None },
            Self::Down => if let location::NextDirection::Down = capture {
                Some(Self::R::Down)
            } else { None },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Vec<T: Query> {
    Any(T)
}

#[derive(Debug, Clone)]
pub enum VecR<R> {
    Index(usize, R)
}

impl<T: Query> Query for Vec<T> {
    type R = VecR<T::R>;
    type C = std::vec::Vec<T::C>;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::Any(query) => {
                let mut catch = None;
                for (idx, item) in capture.iter().enumerate() {
                    if let Some(result) = query.run(item) {
                        catch = Some(Self::R::Index(idx, result));
                        break
                    }
                }
                catch
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
    type R = std::result::Result<T::R, E::R>;
    type C = std::result::Result<T::C, E::C>;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::Ok(query) => if let Ok(capture) = capture {
                Some(Self::R::Ok(query.run(capture)?))
            } else { None },
            Self::Err(query) => if let Err(capture) = capture {
                Some(Self::R::Err(query.run(capture)?))
            } else { None },
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

#[derive(Debug, Clone)]
pub enum LogicR<R> {
    This(R),
    Or(bool, Box<Self>),
    And(Box<Self>, Box<Self>),
    Not
}

impl<T: Query> Query for Logic<T> {
    type R = LogicR<T::R>;
    type C = T::C;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::And(left, right) => if let Some(result_left) = left.run(capture) {
                if let Some(result_right) = right.run(capture) {
                    Some(Self::R::And(Box::new(result_left), Box::new(result_right)))
                } else { None }
            } else { None },
            Self::Or(left, right) => if let Some(r) = left.run(capture) {
                Some(Self::R::Or(false, Box::new(r)))
            } else if let Some(r) = right.run(capture) {
                Some(Self::R::Or(true, Box::new(r)))
            } else { None },
            Self::Not(this) => if let Some(r) = this.run(capture) { None } else {
                Some(Self::R::Not)
            },
            Self::This(this) => Some(Self::R::This(this.run(capture)?))
        }
    }
}

#[derive(Debug, Clone)]
pub enum Cmp<T: Ord> {
    Range(std::ops::Range<T>),
    Equals(T),
}

#[derive(Debug, Clone)]
pub enum CmpR {
    Range,
    Equals
}

impl<T: Ord> Query for Cmp<T> {
    type R = CmpR;
    type C = T;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::Range(range) => if &range.start >= capture && &range.end < capture { Some(Self::R::Range) } else { None },
            Self::Equals(data) => if data == capture { Some(Self::R::Equals) } else { None }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Text {
    Contains(String),
    Is(String),
}

#[derive(Debug, Clone)]
pub enum TextR {
    Range(usize, usize),
    Full
}

impl Query for Text {
    type R = TextR;
    type C = String;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        match self {
            Self::Contains(data) => {
                let start = capture.to_ascii_lowercase().find(data)?;
                Some(Self::R::Range(start, start + data.len()))
            },
            Self::Is(data) => if capture.to_ascii_lowercase() == data.to_ascii_lowercase() { Some(Self::R::Full) } else { None }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bool;

impl Query for Bool {
    type R = ();
    type C = bool;
    fn run(&self, capture: &Self::C) -> std::option::Option<Self::R> {
        if *capture { Some(()) } else { None }
    }
}
