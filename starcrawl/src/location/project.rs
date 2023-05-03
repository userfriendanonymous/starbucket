use std::sync::Arc;
use async_trait::async_trait;
use crate::capture;
use super::{Location, LocationSession};

#[derive(Debug, Clone)]
pub struct Project(pub u64);

#[async_trait]
impl Location for Project {
    type Capture = s2rs::api::Result<s2rs::api::Project>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        session.scratch.project_meta(self.0).await
    }
}
