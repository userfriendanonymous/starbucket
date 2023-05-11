use std::sync::Arc;
use async_trait::async_trait;
use s2rs_derive::Forwarder;
use crate::capture::Capture;
pub use project::*;
pub use studio::*;
pub use user::*;

pub mod project;
pub mod studio;
pub mod user;

#[async_trait]
pub trait Location {
    type Capture: Capture;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture;
}

pub struct LocationSession {
    pub scratch: Arc<s2rs::Api>,
}

impl LocationSession {
    pub fn new(scratch: Arc<s2rs::Api>) -> Arc<Self> {
        Arc::new(Self {
            scratch
        })
    }
}

#[derive(Forwarder, Clone, Debug)]
pub enum CrawlLocation {
    #[forward] User(User),
    #[forward] UserComments(UserComments),
    #[forward] UserProjects(UserProjects),
    #[forward] UserFavorites(UserFavorites),
    #[forward] UserCuratingStudios(UserCuratingStudios),
    #[forward] UserFollowing(UserFollowing),
    #[forward] UserFollowers(UserFollowers),
    #[forward] UserProjectComments(UserProjectComments),

    #[forward] Project(Project),

    #[forward] Studio(Studio),
    #[forward] StudioActivity(StudioActivity),
    #[forward] StudioProjects(StudioProjects),
    #[forward] StudioComments(StudioComments),
}
