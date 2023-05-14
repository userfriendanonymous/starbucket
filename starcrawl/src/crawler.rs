use std::{sync::Arc, collections::{VecDeque, hash_map}, future::Future, hash::{Hash, Hasher}};
use crate::{location::{CrawlLocation, Location, LocationSession}, capture::Capture, output::{Output, CrawlOutput}};

pub struct Crawler {
    api: Arc<s2rs::Api>,
    session: Arc<LocationSession>,
    locations: VecDeque<CrawlLocation>,
    visited: VecDeque<u64>,
    // input: Arc<Input>
}

impl Crawler {
    pub fn new(locations: VecDeque<CrawlLocation>, api: Arc<s2rs::Api>) -> Self {
        let session = LocationSession::new(api.clone());
        Self {
            api,
            locations,
            session,
            visited: VecDeque::new()
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<impl Future<Output = CrawlOutput> + '_> {
        let location = loop {
            let location = self.locations.pop_front()?;

            let mut hasher = hash_map::DefaultHasher::default();
            location.hash(&mut hasher);
            let hash = hasher.finish();
    
            if !self.visited.contains(&hash) {
                if self.visited.len() >= 5000 {
                    self.visited.pop_front();
                }
                self.visited.push_back(hash);
                break location
            }
        };
        
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
