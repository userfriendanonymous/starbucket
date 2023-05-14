use std::{sync::Arc, ops::Add, hash::Hash};
use async_trait::async_trait;
use crate::{capture, UserName};

use super::{Location, LocationSession};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct User(pub UserName);

#[async_trait]
impl Location for User {
    type Capture = s2rs::api::Result<s2rs::api::User>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        session.scratch.user_meta(self.0.as_str()).await
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum NextDirection {
    Up,
    Down,
}

impl Add<usize> for NextDirection {
    type Output = usize;
    fn add(self, rhs: usize) -> Self::Output {
        match self {
            Self::Up => rhs + 1,
            Self::Down => rhs - 1
        }
    }
}

impl Add<u8> for NextDirection {
    type Output = u8;
    fn add(self, rhs: u8) -> Self::Output {
        match self {
            Self::Up => rhs + 1,
            Self::Down => rhs - 1
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

#[derive(Clone, Debug, Hash)]
pub struct UserComments {
    pub page: u8,
    pub name: UserName,
    pub next: Option<NextDirection>,
}


impl UserComments {
    pub fn new(name: UserName, next: Option<NextDirection>) -> Self {
        Self { name, next, page: 0 }
    }

    pub fn new_up(name: UserName) -> Self {
        Self::new(name, Some(NextDirection::Up))
    }
}

#[async_trait]
impl Location for UserComments {
    type Capture = Result<capture::UserComments, s2rs::api::GetUserCommentsError>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        Ok(capture::Chain {
            next: self.next.map(|dir|
                Self { name: self.name.clone(), page: dir + self.page, next: self.next }
            ),
            this: session.scratch.user_comments(self.name.as_str(), Some(self.page)).await?
        })
    }
}

#[derive(Clone, Debug, Hash)]
pub struct UserProjects {
    pub page: usize,
    pub name: UserName,
    pub next: Option<NextDirection>,
}

impl UserProjects {
    pub fn new(name: UserName, next: Option<NextDirection>) -> Self {
        Self { name, next, page: 0 }
    }

    pub fn new_up(name: UserName) -> Self {
        Self::new(name, Some(NextDirection::Up))
    }
}

#[async_trait]
impl Location for UserProjects {
    type Capture = s2rs::api::Result<capture::UserProjects>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        Ok(capture::UserProjects {
            next: self.next.map(|dir|
                Self { name: self.name.clone(), page: dir + self.page, next: self.next }
            ),
            this: session.scratch.user_projects(self.name.as_str(), s2rs::Cursor::with_start(self.page * 40)).await?
        })
    }
}


#[derive(Clone, Debug, Hash)]
pub struct UserFavorites {
    pub page: usize,
    pub name: UserName,
    pub next: Option<NextDirection>,
}

impl UserFavorites {
    pub fn new(name: UserName, next: Option<NextDirection>) -> Self {
        Self { name, next, page: 0 }
    }

    pub fn new_up(name: UserName) -> Self {
        Self::new(name, Some(NextDirection::Up))
    }
}

#[async_trait]
impl Location for UserFavorites {
    type Capture = s2rs::api::Result<capture::UserFavorites>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        Ok(capture::UserFavorites {
            next: self.next.map(|dir|
                Self { name: self.name.clone(), page: dir + self.page, next: self.next }
            ),
            this: session.scratch.user_favorites(self.name.as_str(), s2rs::Cursor::with_start(self.page * 40)).await?
        })
    }
}

#[derive(Clone, Debug, Hash)]
pub struct UserCuratingStudios {
    pub page: usize,
    pub name: UserName,
    pub next: Option<NextDirection>,
}

impl UserCuratingStudios {
    pub fn new(name: UserName, next: Option<NextDirection>) -> Self {
        Self { name, next, page: 0 }
    }

    pub fn new_up(name: UserName) -> Self {
        Self::new(name, Some(NextDirection::Up))
    }
}

#[async_trait]
impl Location for UserCuratingStudios {
    type Capture = s2rs::api::Result<capture::UserCuratingStudios>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        Ok(capture::Chain {
            next: self.next.map(|dir|
                Self { name: self.name.clone(), page: dir + self.page, next: self.next }
            ),
            this: session.scratch.user_curating_studios(self.name.as_str(), s2rs::Cursor::with_start(self.page * 40)).await?
        })
    }
}


#[derive(Clone, Debug, Hash)]
pub struct UserFollowing {
    pub page: usize,
    pub name: UserName,
    pub next: Option<NextDirection>,
}

impl UserFollowing {
    pub fn new(name: UserName, next: Option<NextDirection>) -> Self {
        Self { name, next, page: 0 }
    }

    pub fn new_up(name: UserName) -> Self {
        Self::new(name, Some(NextDirection::Up))
    }
}

#[async_trait]
impl Location for UserFollowing {
    type Capture = s2rs::api::Result<capture::UserFollowing>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        Ok(capture::UserFollowing {
            next: self.next.map(|dir|
                Self { name: self.name.clone(), page: dir + self.page, next: self.next }
            ),
            this: session.scratch.user_following(self.name.as_str(), s2rs::Cursor::with_start(self.page * 40)).await?
        })
    }
}



#[derive(Clone, Debug, Hash)]
pub struct UserFollowers {
    pub page: usize,
    pub name: UserName,
    pub next: Option<NextDirection>,
}

impl UserFollowers {
    pub fn new(name: UserName, next: Option<NextDirection>) -> Self {
        Self { name, next, page: 0 }
    }

    pub fn new_up(name: UserName) -> Self {
        Self::new(name, Some(NextDirection::Up))
    }
}

#[async_trait]
impl Location for UserFollowers {
    type Capture = s2rs::api::Result<capture::UserFollowers>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        Ok(capture::Chain {
            next: self.next.map(|dir|
                Self { name: self.name.clone(), page: dir + self.page, next: self.next }
            ),
            this: session.scratch.user_following(self.name.as_str(), s2rs::Cursor::with_start(self.page * 40)).await?
        })
    }
}

#[derive(Clone, Debug, Hash)]
pub struct UserProjectComments {
    pub page: usize,
    pub name: UserName,
    pub id: u64,
    pub next: Option<NextDirection>,
}

impl UserProjectComments {
    pub fn new_up(name: UserName, id: u64) -> Self {
        Self {
            id,
            name,
            next: Some(NextDirection::Up),
            page: 0
        }
    }
}

#[async_trait]
impl Location for UserProjectComments {
    type Capture = s2rs::api::Result<capture::UserProjectComments>;
    async fn capture(&self, session: Arc<LocationSession>) -> Self::Capture {
        Ok(capture::Chain {
            next: self.next.map(|dir|
                Self { id: self.id, name: self.name.clone(), page: dir + self.page, next: self.next }
            ),
            this: session.scratch.user_project_comments(self.name.as_str(), self.id, s2rs::Cursor::with_start(self.page * 40)).await?
        })
    }
}
