use s2rs_derive::Forwarder;
use crate::location::{Location, self};

#[derive(Debug)]
pub struct Output<L: Location> {
    location: L,
    capture: L::Capture
}

impl<L: Location> Output<L> {
    pub fn new(location: L, capture: L::Capture) -> Self {
        Self {
            capture,
            location
        }
    }
}

#[derive(Forwarder, Debug)]
pub enum CrawlOutput {
    #[forward] User(Output<location::User>),
    #[forward] Project(Box<Output<location::Project>>),
    #[forward] UserComments(Output<location::UserComments>),
    #[forward] UserCommentsScroll(Output<location::UserCommentsScroll>),
    #[forward] UserProjects(Output<location::UserProjects>),
}
