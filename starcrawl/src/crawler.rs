use std::{sync::Arc, collections::VecDeque, future::Future};
use crate::{location::{CrawlLocation, Location, LocationSession}, capture::Capture, output::{Output, CrawlOutput}};

pub struct Crawler {
    api: Arc<s2rs::Api>,
    location_session: Arc<LocationSession>,
    locations: VecDeque<CrawlLocation>,
    // input: Arc<Input>
}

impl Crawler {
    pub fn new(locations: VecDeque<CrawlLocation>, api: Arc<s2rs::Api>) -> Self {
        let location_session = LocationSession::new(api.clone());
        Self {
            api,
            locations,
            location_session
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<impl Future<Output = CrawlOutput> + '_> {
        let location = self.locations.pop_front()?;
        println![ "{:?}", format!["{:?}", &location].split('}').next().unwrap() ];
        Some(async {
            let (locations, output) = Crawl::new(location, self.location_session.clone()).run().await;
            for location in locations {
                self.locations.push_back(location);
            }
            output
        })
    }
}

pub struct Crawl {
    location_session: Arc<LocationSession>,
    location: CrawlLocation,
}

impl Crawl {
    pub fn new(location: CrawlLocation, location_session: Arc<LocationSession>) -> Self {
        Self {
            location,
            location_session
        }
    }

    pub async fn run(self) -> (Vec<CrawlLocation>, CrawlOutput) {
        match self.location {
            CrawlLocation::User(location) => {
                let capture = location.capture(self.location_session.clone()).await;
                (capture.populate(), Output::new(location, capture).into())
            }
            CrawlLocation::UserComments(location) => {
                let capture = location.capture(self.location_session.clone()).await;
                (capture.populate(), Output::new(location, capture).into())
            }
            CrawlLocation::UserProjects(location) => {
                let capture = location.capture(self.location_session.clone()).await;
                (capture.populate(), Output::new(location, capture).into())
            }
            CrawlLocation::UserFavorites(location) => {
                let capture = location.capture(self.location_session.clone()).await;
                (capture.populate(), Output::new(location, capture).into())
            }
            CrawlLocation::UserCuratingStudios(location) => {
                let capture = location.capture(self.location_session.clone()).await;
                (capture.populate(), Output::new(location, capture).into())
            }
            CrawlLocation::UserFollowing(location) => {
                let capture = location.capture(self.location_session.clone()).await;
                (capture.populate(), Output::new(location, capture).into())
            }
            CrawlLocation::UserFollowers(location) => {
                let capture = location.capture(self.location_session.clone()).await;
                (capture.populate(), Output::new(location, capture).into())
            }

            CrawlLocation::Project(location) => {
                let capture = location.capture(self.location_session.clone()).await;
                (capture.populate(), Output::new(location, capture).into())
            }
        }
    }
}
