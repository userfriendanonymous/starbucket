use crate::location::CrawlLocation;

use super::Capture;

#[derive(Debug, Clone)]
pub struct UserMeta {
    pub name: String,
    pub scratch_team: bool,
    pub id: u64,
    pub profile: UserProfile
}

impl Capture for UserMeta {
    fn populate(&self) -> Vec<CrawlLocation> {
        vec![
            // UserMetaLocation("".to_owned()).into()
        ]
    }
}

impl From<s2rs::api::User> for UserMeta {
    fn from(value: s2rs::api::User) -> Self {
        Self {
            profile: value.profile.into(),
            id: value.id,
            name: value.name,
            scratch_team: value.scratch_team
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserProfile {
    pub id: u64,
    pub wiwo: String,
    pub bio: String,
    pub country: String,
}

impl From<s2rs::api::UserProfile> for UserProfile {
    fn from(value: s2rs::api::UserProfile) -> Self {
        Self {
            bio: value.bio,
            country: value.country,
            id: value.id,
            wiwo: value.status
        }
    }
}

pub struct UserProject {
    pub description: String,
    pub instructions: String,
}
