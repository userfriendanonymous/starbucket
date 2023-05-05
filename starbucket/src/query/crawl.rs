use super::{Logic, Cmp, Query, User, Result, Project};


#[derive(Debug)]
pub enum S2rsError {
    Status(Logic<Cmp<u16>>),
    Network,
    Parsing
}

impl Query for S2rsError {
    type C = s2rs::api::Error;
    fn run(&self, capture: &Self::C) -> bool {
        type E = s2rs::api::Error;
        match self {
            Self::Status(query) => if let E::Status(status) = capture {
                query.run(&status.as_u16())
            } else { false },
            Self::Network => matches!(capture, E::Network(..)),
            Self::Parsing => matches!(capture, E::Parsing(..)),
        }
    }
}

// #[derive(Debug)]
// pub struct UserCrawl(Result<User, S2rsError>);

// impl Query for UserCrawl {
//     type C = s2rs::api::Result<s2rs::api::User>;
//     fn run(&self, capture: &Self::C) -> bool {
//         let data = capture.as_ref();
//         self.0.run(&capture.clone().map(|e| e.into()))
//     }
// }

// #[derive(Debug)]
// pub struct ProjectCrawl(Result<Project, S2rsError>);

// impl ProjectCrawl {
//     pub fn run(&self, capture: s2rs::api::Result<s2rs::api::Project>) -> bool {
//         self.0.run(&cap)
//     }
// }

// impl Query for ProjectCrawl {
//     type C = s2rs::api::Result<s2rs::api::Project>;
//     fn run(&self, capture: &Self::C) -> bool {
//         self.0.run(&capture.clone().map(|e| e.into()))
//     }
// }
