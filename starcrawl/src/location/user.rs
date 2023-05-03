use std::sync::Arc;
use async_trait::async_trait;
use crate::capture;

use super::{Location, LocationSession};

#[derive(Debug, Clone)]
pub struct User(pub String);

#[async_trait]
impl Location for User {
    type Capture = s2rs::api::Result<s2rs::api::User>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        session.scratch.user_meta(&self.0).await
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NextDirection {
    Up,
    Down,
}

impl NextDirection {
    pub fn add_u8(self, value: u8) -> u8 {
        match self {
            Self::Up => value + 1,
            Self::Down => value - 1
        }
    }
    
    pub fn add_u32(self, value: u32) -> u32 {
        match self {
            Self::Up => value + 1,
            Self::Down => value - 1
        }
    }
}

impl From<NextDirection> for i8 {
    fn from(value: NextDirection) -> Self {
        match value {
            NextDirection::Down => -1,
            NextDirection::Up => 1
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserComments {
    pub page: u8,
    pub name: String,
    pub next: Option<NextDirection>,
}


impl UserComments {
    pub fn new(name: String, next: Option<NextDirection>) -> Self {
        Self { name, next, page: 0 }
    }

    pub fn new_up(name: String) -> Self {
        Self::new(name, Some(NextDirection::Up))
    }
}

#[async_trait]
impl Location for UserComments {
    type Capture = Result<capture::UserComments, s2rs::api::GetUserCommentsError>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        Ok(capture::Chain {
            next: self.next.map(|dir|
                Self { name: self.name.clone(), page: dir.add_u8(self.page), next: self.next }
            ),
            this: session.scratch.user_comments(&self.name, Some(self.page)).await?
        })
    }
}

#[derive(Debug, Clone)]
pub struct UserProjects {
    pub page: u32,
    pub name: String,
    pub next: Option<NextDirection>,
}

impl UserProjects {
    pub fn new(name: String, next: Option<NextDirection>) -> Self {
        Self { name, next, page: 0 }
    }

    pub fn new_up(name: String) -> Self {
        Self::new(name, Some(NextDirection::Up))
    }
}

#[async_trait]
impl Location for UserProjects {
    type Capture = s2rs::api::Result<capture::UserProjects>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        Ok(capture::UserProjects {
            next: self.next.map(|dir|
                Self { name: self.name.clone(), page: dir.add_u32(self.page), next: self.next }
            ),
            this: session.scratch.user_projects(&self.name, s2rs::Cursor::with_start((self.page * 40) as _)).await?
        })
    }
}


#[derive(Debug, Clone)]
pub struct UserFavorites {
    pub page: u32,
    pub name: String,
    pub next: Option<NextDirection>,
}

impl UserFavorites {
    pub fn new(name: String, next: Option<NextDirection>) -> Self {
        Self { name, next, page: 0 }
    }

    pub fn new_up(name: String) -> Self {
        Self::new(name, Some(NextDirection::Up))
    }
}

#[async_trait]
impl Location for UserFavorites {
    type Capture = s2rs::api::Result<capture::UserFavorites>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        Ok(capture::UserFavorites {
            next: self.next.map(|dir|
                Self { name: self.name.clone(), page: dir.add_u32(self.page), next: self.next }
            ),
            this: session.scratch.user_favorites(&self.name, s2rs::Cursor::with_start((self.page * 40) as _)).await?
        })
    }
}

#[derive(Debug, Clone)]
pub struct UserCuratingStudios {
    pub page: u32,
    pub name: String,
    pub next: Option<NextDirection>,
}

impl UserCuratingStudios {
    pub fn new(name: String, next: Option<NextDirection>) -> Self {
        Self { name, next, page: 0 }
    }

    pub fn new_up(name: String) -> Self {
        Self::new(name, Some(NextDirection::Up))
    }
}

#[async_trait]
impl Location for UserCuratingStudios {
    type Capture = s2rs::api::Result<capture::UserCuratingStudios>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        Ok(capture::Chain {
            next: self.next.map(|dir|
                Self { name: self.name.clone(), page: dir.add_u32(self.page), next: self.next }
            ),
            this: session.scratch.user_curating_studios(&self.name, s2rs::Cursor::with_start((self.page * 40) as _)).await?
        })
    }
}


#[derive(Debug, Clone)]
pub struct UserFollowing {
    pub page: u32,
    pub name: String,
    pub next: Option<NextDirection>,
}

impl UserFollowing {
    pub fn new(name: String, next: Option<NextDirection>) -> Self {
        Self { name, next, page: 0 }
    }

    pub fn new_up(name: String) -> Self {
        Self::new(name, Some(NextDirection::Up))
    }
}

#[async_trait]
impl Location for UserFollowing {
    type Capture = s2rs::api::Result<capture::UserFollowing>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        Ok(capture::UserFollowing {
            next: self.next.map(|dir|
                Self { name: self.name.clone(), page: dir.add_u32(self.page), next: self.next }
            ),
            this: session.scratch.user_following(&self.name, s2rs::Cursor::with_start((self.page * 40) as _)).await?
        })
    }
}



#[derive(Debug, Clone)]
pub struct UserFollowers {
    pub page: u32,
    pub name: String,
    pub next: Option<NextDirection>,
}

impl UserFollowers {
    pub fn new(name: String, next: Option<NextDirection>) -> Self {
        Self { name, next, page: 0 }
    }

    pub fn new_up(name: String) -> Self {
        Self::new(name, Some(NextDirection::Up))
    }
}

#[async_trait]
impl Location for UserFollowers {
    type Capture = s2rs::api::Result<capture::UserFollowers>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        Ok(capture::Chain {
            next: self.next.map(|dir|
                Self { name: self.name.clone(), page: dir.add_u32(self.page), next: self.next }
            ),
            this: session.scratch.user_following(&self.name, s2rs::Cursor::with_start((self.page * 40) as _)).await?
        })
    }
}
