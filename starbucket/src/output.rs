use s2rs_derive::Forwarder;
use starcrawl::location::{Location, self};
use crate::query::{Query, self};

#[derive(Forwarder, Debug)]
pub enum BucketOutput<'a> {
    #[forward] User(Output<'a, location::User, query::UserResult>),
    #[forward] UserComments(Output<'a, location::UserComments, query::UserCommentsResult>),
    #[forward] UserProjects(Output<'a, location::UserProjects, query::UserProjectsResult>),
    #[forward] UserFollowers(Output<'a, location::UserFollowers, query::UserFollowersResult>),
    #[forward] UserFollowing(Output<'a, location::UserFollowing, query::UserFollowingResult>),

    #[forward] Project(Output<'a, location::Project, query::ProjectResult>),
    #[forward] Studio(Output<'a, location::Studio, query::StudioResult>),
    #[forward] StudioActivity(Output<'a, location::StudioActivity, query::StudioActivityResult>),
    #[forward] StudioComments(Output<'a, location::StudioComments, query::StudioCommentsResult>),
    #[forward] StudioProjects(Output<'a, location::StudioProjects, query::StudioProjectsResult>),
    Todo
}

#[derive(Debug)]
pub struct Output<'a, L: Location, Q: Query> {
    pub location: L,
    pub capture: Q::C,
    pub queries: Vec<&'a Q>
}

impl<'a, L: Location, Q: Query> Output<'a, L, Q> {
    pub fn run(capture: Q::C, location: L, queries: &'a [Q]) -> Self {
        let mut matching_queries = Vec::new();
        for query in queries.iter() {
            if query.run(&capture) {
                matching_queries.push(query);
            }
        }

        Self {
            capture,
            location,
            queries: matching_queries
        }
    }
}
