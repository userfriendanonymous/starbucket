use std::sync::Arc;
use async_trait::async_trait;
use s2rs_derive::Forwarder;
use crate::capture::{Capture, self};

#[async_trait]
pub trait Location {
    type Capture: Capture;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture;
}

pub struct LocationSession {
    pub scratch_api: Arc<s2rs::Api>,
}

impl LocationSession {
    pub fn new(scratch_api: Arc<s2rs::Api>) -> Arc<Self> {
        Arc::new(Self {
            scratch_api
        })
    }
}

#[derive(Forwarder)]
pub enum CrawlLocation {
    #[forward] User(User),
    #[forward] UserComments(UserComments),
    #[forward] UserProjects(UserProjects),
    #[forward] Project(Project),
    #[forward] UserCommentsScroll(UserCommentsScroll)
}

#[derive(Debug)]
pub struct User(pub String);

#[async_trait]
impl Location for User {
    type Capture = s2rs::api::Result<s2rs::api::User>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        session.scratch_api.user_meta(&self.0).await
    }
}

#[derive(Debug)]
pub struct Project(pub u64);

#[async_trait]
impl Location for Project {
    type Capture = s2rs::api::Result<s2rs::api::Project>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        session.scratch_api.project_meta(self.0).await
    }
}

#[derive(Debug)]
pub struct UserComments {
    pub page: u32,
    pub name: String,
}

#[async_trait]
impl Location for UserComments {
    type Capture = Result<Vec<s2rs::api::UserComment>, s2rs::api::GetUserCommentsError>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        session.scratch_api.user_comments(&self.name, s2rs::Cursor::with_start((self.page * 40) as _)).await
    }
}

#[derive(Debug)]
pub struct UserCommentsScroll {
    pub page: u32,
    pub name: String,
}

#[async_trait]
impl Location for UserCommentsScroll {
    type Capture = Result<capture::UserCommentsScroll, s2rs::api::GetUserCommentsError>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        Ok(capture::UserCommentsScroll {
            data: session.scratch_api.user_comments(&self.name, s2rs::Cursor::with_start((self.page * 40) as _)).await?,
            name: self.name.clone(),
            page: self.page + 1,
        })
    }
}

#[derive(Debug)]
pub struct UserProjects {
    pub page: u32,
    pub name: String,
}

#[async_trait]
impl Location for UserProjects {
    type Capture = s2rs::api::Result<Vec<s2rs::api::Project3>>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        session.scratch_api.user_projects(&self.name, s2rs::Cursor::with_start((self.page * 40) as _)).await
    }
}
