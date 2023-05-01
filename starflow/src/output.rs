use std::sync::Arc;
use s2rs_derive::Forwarder;

use crate::{location::{Location, UserMetaLocation}, query::{Query, self}};

#[derive(Forwarder, Debug)]
pub enum CrawlOutput {
    #[forward] UserMeta(Output<UserMetaLocation, query::UserMetaResult>),
}

#[derive(Debug)]
pub struct Output<L: Location, Q: Query<C = Result<L::T, L::E>>> {
    pub queries: Vec<Arc<Q>>,
    pub location: L,
    pub capture: Q::C
}

impl<L: Location, Q: Query<C = Result<L::T, L::E>>> Output<L, Q> {
    pub fn new(capture: Q::C, queries: Arc<Vec<Arc<Q>>>, location: L) -> Self {
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
}