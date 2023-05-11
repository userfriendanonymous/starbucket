
pub use project::*;
pub use studio::*;
pub use user::*;
pub use comment::*;

pub mod project;
pub mod studio;
pub mod user;
pub mod comment;

use crate::location::CrawlLocation;

pub trait Capture {
    fn populate(&self) -> Vec<CrawlLocation>;
}

impl<T: Capture, E: Capture> Capture for Result<T, E> {
    fn populate(&self) -> Vec<CrawlLocation> {
        match self {
            Ok(data) => data.populate(),
            Err(data) => data.populate()
        }
    }
}

impl<T: Capture> Capture for Vec<T> {
    fn populate(&self) -> Vec<CrawlLocation> {
        let mut result = Vec::new();
        for item in self {
            result.append(&mut item.populate());
        }
        result
    }
}

impl Capture for s2rs::api::Error {
    fn populate(&self) -> Vec<CrawlLocation> {
        vec![]
    }
}
