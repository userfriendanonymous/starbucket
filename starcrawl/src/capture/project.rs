use crate::location::{CrawlLocation, self};
use super::Capture;

impl Capture for s2rs::api::Project {
    fn populate(&self) -> Vec<CrawlLocation> {
        vec![
            location::User(self.author.name.clone()).into(),
            location::UserComments::new_up(self.author.name.clone()).into(),
            location::UserProjects::new_up(self.author.name.clone()).into(),
        ]
    }
}

impl Capture for s2rs::api::Project3 {
    fn populate(&self) -> Vec<CrawlLocation> {
        vec![
        ]
    }
}
