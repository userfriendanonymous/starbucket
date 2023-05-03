use crate::location::{CrawlLocation, self, NextDirection, Location};
use super::Capture;

impl Capture for s2rs::api::User {
    fn populate(&self) -> Vec<CrawlLocation> {
        vec![
        ]
    }
}

pub type UserComments = Chain<Vec<s2rs::api::UserComment>, location::UserComments>;
pub type UserProjects = Chain<Vec<s2rs::api::Project3>, location::UserProjects>;
pub type UserFavorites = Chain<Vec<s2rs::api::Project3>, location::UserFavorites>;
pub type UserCuratingStudios = Chain<Vec<s2rs::api::Studio2>, location::UserCuratingStudios>;
pub type UserFollowing = Chain<Vec<s2rs::api::User>, location::UserFollowing>;
pub type UserFollowers = Chain<Vec<s2rs::api::User>, location::UserFollowers>;

impl Capture for s2rs::api::UserComment {
    fn populate(&self) -> Vec<CrawlLocation> {
        vec![
            location::User(self.author_name.clone()).into(),
            location::UserProjects::new_up(self.author_name.clone()).into(),
            location::UserComments::new_up(self.author_name.clone()).into(),
        ]
    }
}

impl Capture for s2rs::api::GetUserCommentsError {
    fn populate(&self) -> Vec<CrawlLocation> {
        vec![]
    }
}

#[derive(Debug)]
pub struct Chain<T: Capture, L: Into<CrawlLocation> + Clone> {
    pub this: T,
    pub next: Option<L>
}

impl<T: Capture, L: Into<CrawlLocation> + Clone> Capture for Chain<T, L> {
    fn populate(&self) -> Vec<CrawlLocation> {
        let mut locations = self.this.populate();
        if let Some(location) = &self.next {
            locations.push(location.clone().into())
        }
        locations
    }
}