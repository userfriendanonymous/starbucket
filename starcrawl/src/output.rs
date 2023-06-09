use s2rs_derive::Forwarder;
use crate::location::{Location, self};

#[derive(Debug)]
pub struct Output<L: Location> {
    pub location: L,
    pub capture: L::Capture
}

impl<L: Location> Output<L> {
    pub fn new(location: L, capture: L::Capture) -> Self {
        Self {
            capture,
            location
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Forwarder, Debug)]
pub enum CrawlOutput {
    #[forward] User(Output<location::User>),
    #[forward] UserComments(Output<location::UserComments>),
    #[forward] UserProjects(Output<location::UserProjects>),
    #[forward] UserFavorites(Output<location::UserFavorites>),
    #[forward] UserCuratingStudios(Output<location::UserCuratingStudios>),
    #[forward] UserFollowing(Output<location::UserFollowing>),
    #[forward] UserFollowers(Output<location::UserFollowers>),
    #[forward] UserProjectComments(Output<location::UserProjectComments>),

    #[forward] Project(Output<location::Project>),

    #[forward] Studio(Output<location::Studio>),
    #[forward] StudioActivity(Output<location::StudioActivity>),
    #[forward] StudioProjects(Output<location::StudioProjects>),
    #[forward] StudioComments(Output<location::StudioComments>),
}
