
pub use user::*;
pub use comment::*;
pub use project::*;

use crate::location::{CrawlLocation};

pub mod user;
pub mod comment;
pub mod project;

pub trait Capture {
    fn populate(&self) -> Vec<CrawlLocation> {
        Vec::new()
    }
}

impl<T: Capture, E: Capture> Capture for Result<T, E> {
    fn populate(&self) -> Vec<CrawlLocation> {
        match self {
            Ok(data) => data.populate(),
            Err(err) => err.populate()
        }
    }
}

impl Capture for s2rs::api::Error {}
