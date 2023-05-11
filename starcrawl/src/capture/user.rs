use s2rs::api::CommentContentFragment;

use crate::{location::{CrawlLocation, self}, active_text::PopulateActives};
use super::Capture;

pub fn populate_user(name: &str) -> Vec<CrawlLocation> {
    vec![
        location::User(name.to_owned()).into(),
        location::UserFollowing::new_up(name.to_owned()).into(),
        location::UserProjects::new_up(name.to_owned()).into(),
        location::UserFavorites::new_up(name.to_owned()).into(),
        location::UserCuratingStudios::new_up(name.to_owned()).into(),
        location::UserComments::new_up(name.to_owned()).into(),
        location::UserFollowers::new_up(name.to_owned()).into(),
    ]
}

impl Capture for s2rs::api::User {
    fn populate(&self) -> Vec<CrawlLocation> {
        let mut items = vec![];
        items.append(&mut self.profile.bio.populate_actives());
        items.append(&mut self.profile.status.populate_actives());
        items
    }
}

pub type UserComments = Chain<Vec<s2rs::api::UserComment>, location::UserComments>;
pub type UserProjects = Chain<Vec<s2rs::api::Project3>, location::UserProjects>;
pub type UserFavorites = Chain<Vec<s2rs::api::Project3>, location::UserFavorites>;
pub type UserCuratingStudios = Chain<Vec<s2rs::api::Studio2>, location::UserCuratingStudios>;
pub type UserFollowing = Chain<Vec<s2rs::api::User>, location::UserFollowing>;
pub type UserFollowers = Chain<Vec<s2rs::api::User>, location::UserFollowers>;
pub type UserProjectComments = Chain<Vec<s2rs::api::Comment>, location::UserProjectComments>;

impl Capture for s2rs::api::UserComment {
    fn populate(&self) -> Vec<CrawlLocation> {
        let mut items = vec![];
        items.append(&mut populate_user(&self.author_name));
        items.append(&mut self.content.0.populate());
        items
    }
}

impl Capture for CommentContentFragment {
    fn populate(&self) -> Vec<CrawlLocation> {
        match self {
            CommentContentFragment::Link { to, content } => {
                let mut items = content.populate_actives();
                items.append(&mut to.populate_actives());
                items
            },
            CommentContentFragment::Emoji(_) => vec![],
            CommentContentFragment::Text(content) => content.populate_actives()
        }
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