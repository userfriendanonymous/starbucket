use std::ops::{Add, Index};

use crate::{capture::{Capture, populate_user, populate_studio}, location::{CrawlLocation, self, populate_project}};

const USER_CHARS: &str = "qazwsxedcrfvtgbyhnujmikolpQAZWSXEDCRFVTGBYHNUJMIKOLP1234567890-_";
const DIGITS: &str = "1234567890";

pub trait PopulateActives {
    fn populate_actives(&self) -> Vec<CrawlLocation>;
}

impl PopulateActives for String {
    fn populate_actives(&self) -> Vec<CrawlLocation> {
        Actives::extract_str(self).populate()
    }
}

#[derive(Default)]
pub struct Actives {
    pub user_links: Vec<String>,
    pub project_links: Vec<u64>,
    pub studio_links: Vec<u64>,
    pub user_tags: Vec<String>,
}

impl Capture for Actives {
    fn populate(&self) -> Vec<CrawlLocation> {
        let mut items = Vec::new();
        for name in &self.user_links {
            items.append(&mut populate_user(name))
        }
        for name in &self.user_tags {
            items.append(&mut populate_user(name))
        }
        for id in &self.project_links {
            items.append(&mut populate_project(id))
        }
        for id in &self.studio_links {
            items.append(&mut populate_studio(id))
        }
        items
    }
}

impl Actives {
    pub fn extract_str(value: &str) -> Actives {
        Self::extract(&value.chars().collect::<Vec<_>>())
    }

    pub fn extract(value: &[char]) -> Actives {
        let user_link: Vec<_> = "scratch.mit.edu/users/".chars().collect();
        let project_link: Vec<_> = "scratch.mit.edu/projects/".chars().collect();
        let studio_link: Vec<_> = "scratch.mit.edu/studios/".chars().collect();
        let user_tag: Vec<_> = "@".chars().collect();
    
        let mut actives = Actives::default();
        let mut value = value;
    
        fn user<'a>(slice: &'a [char], actives: &mut Actives) -> &'a [char] {
            let (name, slice) = cut_on_wrong(slice, |c| USER_CHARS.contains(*c));
            if name.len() > 1 {
                actives.user_links.push(name.iter().collect());
            }
            slice
        }
    
        while !value.is_empty() {
            if let Some(slice) = value.strip_prefix(user_link.as_slice()) {
                value = user(slice, &mut actives);
    
            } else if let Some(slice) = value.strip_prefix(user_tag.as_slice()) {
                value = user(slice, &mut actives);
    
            } else if let Some(slice) = value.strip_prefix(project_link.as_slice()) {
                let (id, slice) = cut_on_wrong(slice, |c| DIGITS.contains(*c));
                value = slice;
                if id.len() > 2 {
                    if let Ok(id) = id.iter().collect::<String>().parse() {
                        actives.project_links.push(id);
                    }
                }
    
            } else if let Some(slice) = value.strip_prefix(studio_link.as_slice()) {
                let (id, slice) = cut_on_wrong(slice, |c| DIGITS.contains(*c));
                value = slice;
                if id.len() > 2 {
                    if let Ok(id) = id.iter().collect::<String>().parse() {
                        actives.studio_links.push(id);
                    }
                }
    
            } else {
                value = match value.get(1..) {
                    Some(value) => value,
                    None => break
                };
            }
        }
    
        actives
    }
}

fn cut_on_wrong(value: &'_ [char], f: impl Fn(&char) -> bool) -> (&'_ [char], &'_ [char]) {
    for (idx, item) in value.iter().enumerate() {
        if !f(item) {
            return value.split_at(idx);
        }
    }
    (value, &[])
}