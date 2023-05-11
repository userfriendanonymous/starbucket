use crate::{location::CrawlLocation, active_text::PopulateActives};
use super::{Capture, populate_user};

impl Capture for s2rs::api::Project {
    fn populate(&self) -> Vec<CrawlLocation> {
        let mut items = vec![];
        items.append(&mut populate_user(&self.author.name));
        items.append(&mut self.description.populate_actives());
        items.append(&mut self.instructions.populate_actives());
        items.append(&mut self.title.populate_actives());
        items
    }
}

impl Capture for s2rs::api::Project3 {
    fn populate(&self) -> Vec<CrawlLocation> {
        let mut items = vec![];
        items.append(&mut self.description.populate_actives());
        items.append(&mut self.instructions.populate_actives());
        items.append(&mut self.title.populate_actives());
        items
    }
}
