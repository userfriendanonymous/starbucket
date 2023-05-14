use crate::{location::{CrawlLocation, self}, active_text::PopulateActives};
use super::{Capture, Chain, populate_user};

pub type StudioActivity = Chain<Vec<s2rs::api::StudioAction>, location::StudioActivity>;
pub type StudioProjects = Chain<Vec<s2rs::api::StudioProject>, location::StudioProjects>;
pub type StudioComments = Chain<Vec<s2rs::api::Comment>, location::StudioComments>;

pub fn populate_studio(id: &u64) -> Vec<CrawlLocation> {
    vec![
        location::Studio(*id).into(),
        location::StudioActivity::new_up(*id).into()
    ]
}

impl Capture for s2rs::api::Studio2 {
    fn populate(&self) -> Vec<CrawlLocation> {
        let mut items = vec![];
        items.append(&mut self.description.populate_actives());
        items.append(&mut self.title.populate_actives());
        items
    }
}

impl Capture for s2rs::api::Studio {
    fn populate(&self) -> Vec<CrawlLocation> {
        let mut items = vec![];
        items.append(&mut self.description.populate_actives());
        items.append(&mut self.title.populate_actives());
        items
    }
}

impl Capture for s2rs::api::StudioAction {
    fn populate(&self) -> Vec<CrawlLocation> {
        let mut items = vec![];
        items.append(&mut populate_user(&self.actor_name));
        items
    }
}

impl Capture for s2rs::api::GetStudioActivityError {
    fn populate(&self) -> Vec<CrawlLocation> {
        vec![

        ]
    }
}

impl Capture for s2rs::api::StudioProject {
    fn populate(&self) -> Vec<CrawlLocation> {
        let mut items = vec![
            location::Project(self.id).into()
        ];
        items.append(&mut self.title.populate_actives());
        items.append(&mut populate_user(&self.name));
        items
    }
}
