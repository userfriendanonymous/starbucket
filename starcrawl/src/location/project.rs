use std::sync::Arc;
use async_trait::async_trait;
use crate::{location, capture};
use super::{Location, LocationSession, CrawlLocation, NextDirection};

pub fn populate_project(id: &u64) -> Vec<CrawlLocation> {
    vec![
        location::Project(*id).into()
    ]
}

#[derive(Debug, Clone, Hash)]
pub struct Project(pub u64);

#[async_trait]
impl Location for Project {
    type Capture = s2rs::api::Result<s2rs::api::Project>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        session.scratch.project_meta(self.0).await
    }
}
