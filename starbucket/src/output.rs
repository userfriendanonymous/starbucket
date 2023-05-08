use std::sync::Arc;
use s2rs_derive::Forwarder;
use starcrawl::location::{Location, self};
use crate::query::{Query, self};

#[derive(Forwarder, Debug)]
pub enum BucketOutput {
    #[forward] User(Output<location::User, query::UserResult>),
    #[forward] Project(Output<location::Project, query::ProjectResult>),
    #[forward] UserComments(Output<location::UserComments, query::UserCommentsResult>),
    #[forward] UserProjects(Output<location::UserProjects, query::UserProjectsResult>),
    #[forward] UserFollowers(Output<location::UserFollowers, query::UserFollowersResult>),
    #[forward] UserFollowing(Output<location::UserFollowing, query::UserFollowingResult>),
    Todo
}

#[derive(Debug)]
pub struct Output<L: Location, Q: Query> {
    pub location: L,
    pub capture: Q::C,
    pub queries: Vec<Arc<Q>>
}

impl<L: Location, Q: Query> Output<L, Q> {
    pub fn run(capture: Q::C, location: L, queries: Arc<Vec<Arc<Q>>>) -> Self {
        let mut matching_queries = Vec::new();
        for query in queries.iter() {
            if query.run(&capture) {
                matching_queries.push(query.clone());
            }
        }

        Self {
            capture,
            location,
            queries: matching_queries
        }
    }

    pub fn from_crawl_output(output: starcrawl::output::Output<L>, queries: Arc<Vec<Arc<Q>>>) -> Self
    where L::Capture: Into<Q::C> {
        Self::run(output.capture.into(), output.location, queries)
    }
}
