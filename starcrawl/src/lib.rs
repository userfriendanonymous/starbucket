
pub mod location;
pub mod crawler;
pub mod active_text;
pub mod capture;
pub mod output;

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::{crawler::Crawler, location::{User, UserProjects, Project}};

    #[tokio::test]
    async fn main() -> Result<(), ()> {
        let api = s2rs::Api::new("");

        let mut locations = VecDeque::new();
        
        locations.push_back(Project(659001424).into());

        let mut crawler = Crawler::new(locations, api);

        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            let output = crawler.next().unwrap().await;
            dbg![
                output
            ];
        }

        Ok(())
    }
}