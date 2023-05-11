use std::{sync::Arc, collections::VecDeque, future::Future};
use crate::{location::{CrawlLocation, Location, LocationSession}, capture::Capture, output::{Output, CrawlOutput}};

pub struct Crawler {
    api: Arc<s2rs::Api>,
    session: Arc<LocationSession>,
    locations: VecDeque<CrawlLocation>,
    // input: Arc<Input>
}

impl Crawler {
    pub fn new(locations: VecDeque<CrawlLocation>, api: Arc<s2rs::Api>) -> Self {
        let session = LocationSession::new(api.clone());
        Self {
            api,
            locations,
            session
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<impl Future<Output = CrawlOutput> + '_> {
        let location = self.locations.pop_front()?;
        println![ "{:?}", format!["{:?}", &location].split('}').next().unwrap() ];
        Some(async {
            let (locations, output) = Crawl::new(location, self.session.clone()).run().await;
            for location in locations {
                self.locations.push_back(location);
            }
            output
        })
    }
}

async fn run_location<L: Location>(location: L, session: Arc<LocationSession>) -> (Vec<CrawlLocation>, CrawlOutput) where Output<L>: Into<CrawlOutput> {
    let capture = location.capture(session).await;
    (capture.populate(), Output::new(location, capture).into())
}

pub struct Crawl {
    session: Arc<LocationSession>,
    location: CrawlLocation,
}

impl Crawl {
    pub fn new(location: CrawlLocation, session: Arc<LocationSession>) -> Self {
        Self {
            location,
            session
        }
    }

    pub async fn run(self) -> (Vec<CrawlLocation>, CrawlOutput) {
        match self.location {
            CrawlLocation::User(location) => run_location(location, self.session.clone()).await,
            CrawlLocation::UserComments(location) => run_location(location, self.session.clone()).await,
            CrawlLocation::UserProjects(location) => run_location(location, self.session.clone()).await,
            CrawlLocation::UserFavorites(location) => run_location(location, self.session.clone()).await,
            CrawlLocation::UserCuratingStudios(location) => run_location(location, self.session.clone()).await,
            CrawlLocation::UserFollowing(location) => run_location(location, self.session.clone()).await,
            CrawlLocation::UserFollowers(location) => run_location(location, self.session.clone()).await,
            CrawlLocation::UserProjectComments(location) => run_location(location, self.session.clone()).await,

            CrawlLocation::Project(location) => run_location(location, self.session.clone()).await,

            CrawlLocation::Studio(location) => run_location(location, self.session.clone()).await,
            CrawlLocation::StudioActivity(location) => run_location(location, self.session.clone()).await,
            CrawlLocation::StudioComments(location) => run_location(location, self.session.clone()).await,
            CrawlLocation::StudioProjects(location) => run_location(location, self.session.clone()).await,
        }
    }
}
