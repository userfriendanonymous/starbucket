// mod entities;
mod query;
mod output;
mod input;
mod bucket;

use std::{collections::VecDeque, sync::Arc};
use input::{Input, InputQueries};
use query::Query;
use output::{Output, BucketOutput};
use starcrawl::{crawler::Crawler, location, output::CrawlOutput};
use bucket::Bucket;


#[cfg(test)]
#[tokio::test]
#[allow(clippy::single_match)]
async fn test() {
    use starcrawl::location::NextDirection;

    let input: Input = Input {
        queries: InputQueries {
            user_followers: Arc::new(Vec::new()),
            user_following: Arc::new(Vec::new()),
            project: Arc::new(vec![
                Arc::new(
                    query::Result::Ok(
                        query::Logic::Or(
                            Box::new(query::Logic::This(
                                query::Project::Description(query::Logic::Or(
                                    Box::new(query::Logic::Or(
                                        Box::new(query::Logic::This(
                                            query::Text::Contains("Rosyda".to_owned())
                                        )),
                                        Box::new(query::Logic::This(
                                            query::Text::Contains("x__0".to_owned())
                                        ))
                                    )),
                                    Box::new(query::Logic::Or(
                                        Box::new(query::Logic::This(
                                            query::Text::Contains("Finai".to_owned())
                                        )),
                                        Box::new(query::Logic::This(
                                            query::Text::Contains("Dyaros".to_owned())
                                        ))
                                    )),
                                ))
                            ))
                            , 
                            Box::new(query::Logic::This(
                                query::Project::Instructions(query::Logic::Or(
                                    Box::new(query::Logic::Or(
                                        Box::new(query::Logic::This(
                                            query::Text::Contains("Rosyda".to_owned())
                                        )),
                                        Box::new(query::Logic::This(
                                            query::Text::Contains("x__0".to_owned())
                                        ))
                                    )),
                                    Box::new(query::Logic::Or(
                                        Box::new(query::Logic::This(
                                            query::Text::Contains("Finai".to_owned())
                                        )),
                                        Box::new(query::Logic::This(
                                            query::Text::Contains("Dyaros".to_owned())
                                        ))
                                    )),
                                ))
                            ))
                        )
                    )
                )
            ]),
            user_projects: Arc::new(Vec::new()),
            user_comments: Arc::new(vec![
                Arc::new(
                    query::Result::Ok(query::Logic::This(
                        query::Chain::This(query::Logic::This(
                            query::Vec::Any(query::Logic::Or(
                                Box::new(query::Logic::Or(
                                    Box::new(query::Logic::This(
                                        query::UserComment::AuthorName(query::Logic::This(
                                            query::Text::Is("Rosyda".to_string())
                                        ))
                                    )),
                                    Box::new(query::Logic::This(
                                        query::UserComment::AuthorName(query::Logic::This(
                                            query::Text::Is("Dyaros".to_string())
                                        ))
                                    )),
                                )),
                                Box::new(query::Logic::Or(
                                    Box::new(query::Logic::This(
                                        query::UserComment::AuthorName(query::Logic::This(
                                            query::Text::Is("x__0".to_string())
                                        ))
                                    )),
                                    Box::new(query::Logic::This(
                                        query::UserComment::AuthorName(query::Logic::This(
                                            query::Text::Is("FinaI".to_string())
                                        ))
                                    )),
                                ))
                            ))
                        ))
                    ))
                )
            ]),
            user: Arc::new(vec![
                Arc::new(
                    query::Result::Ok(query::Logic::This(
                        query::User::Profile(query::UserProfile::Bio(
                            query::Logic::Or(
                                Box::new(query::Logic::Or(
                                    Box::new(query::Logic::This(
                                        query::Text::Contains("Rosyda".to_string())
                                    )),
                                    Box::new(query::Logic::This(
                                        query::Text::Contains("x__0".to_string())
                                    )),
                                )),
                                Box::new(query::Logic::Or(
                                    Box::new(query::Logic::This(
                                        query::Text::Contains("Dyaros".to_string())
                                    )),
                                    Box::new(query::Logic::This(
                                        query::Text::Contains("FinaI".to_string())
                                    )),
                                )),
                            )
                        ))
                    ))
                )
            ]),

        }
    };
    let api = s2rs::Api::new("");

    let mut locations = VecDeque::new();
    locations.push_back(location::User("Rosyda".to_owned()).into());
    locations.push_back(location::UserProjects {
        name: "Rosyda".to_owned(),
        next: Some(NextDirection::Up),
        page: 1
    }.into());
    locations.push_back(location::UserFollowing {
        name: "Rosyda".to_owned(),
        next: Some(NextDirection::Up),
        page: 1
    }.into());

    locations.push_back(location::User("evelynn-".to_owned()).into());
    locations.push_back(location::UserProjects {
        name: "evelynn-".to_owned(),
        next: Some(NextDirection::Up),
        page: 1
    }.into());
    locations.push_back(location::UserFollowers {
        name: "evelynn-".to_owned(),
        next: Some(NextDirection::Up),
        page: 1
    }.into());

    locations.push_back(location::User("x__0".to_owned()).into());
    locations.push_back(location::UserProjects {
        name: "x__0".to_owned(),
        next: Some(NextDirection::Up),
        page: 1
    }.into());
    
    let mut crawler = Crawler::new(locations, api);

    let mut bucket = Bucket {
        crawler,
        input
    };

    while let Some(f) = bucket.next() {
        println!("Requesting - ");
        let data = f.await;
        match data {
            BucketOutput::Project(data) => if !data.queries.is_empty() {dbg![ &data ];},
            BucketOutput::User(data) => if !data.queries.is_empty() {dbg![ &data ];},
            BucketOutput::UserComments(data) => if !data.queries.is_empty() {dbg![ &data ];},
            BucketOutput::UserProjects(data) => if !data.queries.is_empty() {dbg![ &data ];},
            _ => { dbg![ "todo" ]; },
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }
}
