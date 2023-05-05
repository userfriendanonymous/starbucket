mod entities;
mod query;
mod output;
mod input;

use std::{collections::VecDeque, sync::Arc};
use input::{Input, InputQueries};
use query::Query;
use output::{Output, BucketOutput};
use starcrawl::{crawler::Crawler, location::User, output::CrawlOutput};

#[cfg(test)]
#[tokio::test]
#[allow(clippy::single_match)]
async fn test() {
    let input: Input = Input {
        queries: InputQueries {
            project: Arc::new(Vec::new()),
            user: Arc::new(vec![
                Arc::new(
                    query::Result::Ok(query::Logic::This(
                        query::User::Profile(query::UserProfile::Bio(
                            query::Logic::This(query::Text::Contains("a".to_string()))
                        ))
                    ))
                )
            ]),
        }
    };
    let api = s2rs::Api::new("");

    let mut locations = VecDeque::new();
    locations.push_back(User("griffpatch".to_owned()).into());
    
    let mut crawler = Crawler::new(locations, api);

    let mut outputs = Vec::<BucketOutput>::new();

    while let Some(future) = crawler.next() {
        let output = future.await;
        match output {
            CrawlOutput::Project(output) => {
                let capture = output.capture.map(|e| e.into());
                outputs.push(Output::run(capture, input.queries.project.clone(), output.location).into());
            },
            CrawlOutput::User(output) => {
                print!("co");
                let capture = output.capture.map(|e| e.into());
                dbg![ Output::run(capture, input.queries.user.clone(), output.location) ];
            },
            _ => {}
        }
    }
}
