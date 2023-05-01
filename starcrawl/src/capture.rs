use crate::location::{CrawlLocation, self, Location};

#[derive(Debug)]
pub struct UserCommentsScroll {
    pub page: u32,
    pub name: String,
    pub data: Vec<s2rs::api::UserComment>,
}

impl Capture for UserCommentsScroll {
    fn populate(&self) -> Vec<CrawlLocation> {
        let mut locations = self.data.populate();
        locations.push(location::UserComments {
            page: self.page,
            name: self.name.clone(),
        }.into());
        locations
    }
}

pub trait Capture {
    fn populate(&self) -> Vec<CrawlLocation>;
}

impl<T: Capture, E: Capture> Capture for Result<T, E> {
    fn populate(&self) -> Vec<CrawlLocation> {
        match self {
            Ok(data) => data.populate(),
            Err(data) => data.populate()
        }
    }
}

impl<T: Capture> Capture for Vec<T> {
    fn populate(&self) -> Vec<CrawlLocation> {
        let mut result = Vec::new();
        for item in self {
            result.append(&mut item.populate());
        }
        result
    }
}

impl Capture for s2rs::api::Project {
    fn populate(&self) -> Vec<CrawlLocation> {
        vec![
            location::User(self.author.name.clone()).into(),
        ]
    }
}

impl Capture for s2rs::api::Project3 {
    fn populate(&self) -> Vec<CrawlLocation> {
        vec![
            // location::UserProjects(self.id).into()
        ]
    }
}


impl Capture for s2rs::api::User {
    fn populate(&self) -> Vec<CrawlLocation> {
        vec![
        ]
    }
}

impl Capture for s2rs::api::UserComment {
    fn populate(&self) -> Vec<CrawlLocation> {
        vec![
            location::User(self.author_name.clone()).into()
        ]
    }
}

impl Capture for s2rs::api::Error {
    fn populate(&self) -> Vec<CrawlLocation> {
        vec![]
    }
}

impl Capture for s2rs::api::GetUserCommentsError {
    fn populate(&self) -> Vec<CrawlLocation> {
        vec![]
    }
}
