use crate::active_text::PopulateActives;
use super::{Capture, populate_user};

impl Capture for s2rs::api::Comment {
    fn populate(&self) -> Vec<crate::location::CrawlLocation> {
        let mut items = vec![];
        items.append(&mut self.content.populate_actives());
        items.append(&mut populate_user(&self.author.name));
        items
    }
}
