use std::sync::Arc;
use async_trait::async_trait;
use crate::capture;
use super::{Location, LocationSession, NextDirection};


#[derive(Debug, Clone)]
pub struct Studio(pub u64);

#[async_trait]
impl Location for Studio {
    type Capture = s2rs::api::Result<s2rs::api::Studio>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        session.scratch.studio_meta(self.0).await
    }
}

#[derive(Clone, Debug, Hash)]
pub struct StudioActivity {
    pub page: usize,
    pub id: u64,
    pub next: Option<NextDirection>,
}

impl StudioActivity {
    pub fn new_up(id: u64) -> Self {
        Self {
            id,
            next: Some(NextDirection::Up),
            page: 0
        }
    }
}

#[async_trait]
impl Location for StudioActivity {
    type Capture = Result<capture::StudioActivity, s2rs::api::GetStudioActivityError>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        Ok(capture::Chain {
            next: self.next.map(|dir|
                Self { id: self.id, page: dir + self.page, next: self.next }
            ),
            this: session.scratch.studio_activity(self.id, s2rs::Cursor::with_start(self.page * 40)).await?
        })
    }
}


#[derive(Clone, Debug, Hash)]
pub struct StudioProjects {
    pub page: usize,
    pub id: u64,
    pub next: Option<NextDirection>,
}

impl StudioProjects {
    pub fn new_up(id: u64) -> Self {
        Self {
            id,
            next: Some(NextDirection::Up),
            page: 0
        }
    }
}

#[async_trait]
impl Location for StudioProjects {
    type Capture = s2rs::api::Result<capture::StudioProjects>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        Ok(capture::Chain {
            next: self.next.map(|dir|
                Self { id: self.id, page: dir + self.page, next: self.next }
            ),
            this: session.scratch.studio_projects(self.id, s2rs::Cursor::with_start(self.page * 40)).await?
        })
    }
}

#[derive(Clone, Debug, Hash)]
pub struct StudioComments {
    pub page: usize,
    pub id: u64,
    pub next: Option<NextDirection>,
}

impl StudioComments {
    pub fn new_up(id: u64) -> Self {
        Self {
            id,
            next: Some(NextDirection::Up),
            page: 0
        }
    }
}

#[async_trait]
impl Location for StudioComments {
    type Capture = s2rs::api::Result<capture::StudioComments>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        Ok(capture::Chain {
            next: self.next.map(|dir|
                Self { id: self.id, page: dir + self.page, next: self.next }
            ),
            this: session.scratch.studio_comments(self.id, s2rs::Cursor::with_start(self.page * 40)).await?
        })
    }
}
