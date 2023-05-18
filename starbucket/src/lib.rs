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
    use std::fs::File;
    use std::io::Write;
    use starcrawl::location::NextDirection;

    let input: Input = Input {
        queries: InputQueries {
            user_followers: vec![],
            user_following: vec![],
            studio: vec![
                query::Result::Ok(query::Logic::This(
                    query::Studio::Text(query::Logic::Or(
                        Box::new(query::Logic::Or(
                            Box::new(query::Logic::This(
                                query::Text::Contains("fffffffffffffffffffffffffffffffff".into())
                            )),
                            Box::new(query::Logic::This(
                                query::Text::Contains("x__0".into())
                            ))
                        )),
                        Box::new(query::Logic::Or(
                            Box::new(query::Logic::This(
                                query::Text::Contains("Finai".into())
                            )),
                            Box::new(query::Logic::This(
                                query::Text::Contains("Dyaros".into())
                            ))
                        )),
                    ))
                ))
            ],
            studio_activity: vec![
                query::Result::Ok(query::Logic::This(
                    query::Chain::This(query::Logic::This(
                        query::Vec::Any(query::Logic::This(
                            query::StudioAction::ActorName(query::Logic::Or(
                                Box::new(query::Logic::Or(
                                    Box::new(query::Logic::This(
                                        query::Text::Contains("fffffffffffffffffffffffffffffffff".into())
                                    )),
                                    Box::new(query::Logic::This(
                                        query::Text::Contains("x__0".into())
                                    ))
                                )),
                                Box::new(query::Logic::Or(
                                    Box::new(query::Logic::This(
                                        query::Text::Contains("Finai".into())
                                    )),
                                    Box::new(query::Logic::This(
                                        query::Text::Contains("Dyaros".into())
                                    ))
                                )),
                            )),
                        ))
                    ))
                ))
            ],
            studio_comments: vec![],
            studio_projects: vec![],
            project: vec![
                query::Result::Ok(query::Logic::This(
                    query::Project::Text(query::Logic::Or(
                        Box::new(query::Logic::Or(
                            Box::new(query::Logic::This(
                                query::Text::Contains("fffffffffffffffffffffffffffffffff".into())
                            )),
                            Box::new(query::Logic::This(
                                query::Text::Contains("x__0".into())
                            ))
                        )),
                        Box::new(query::Logic::Or(
                            Box::new(query::Logic::This(
                                query::Text::Contains("Finai".into())
                            )),
                            Box::new(query::Logic::This(
                                query::Text::Contains("Dyaros".into())
                            ))
                        )),
                    ))
                ))
            ],
            user_projects: vec![
                query::Result::Ok(query::Logic::This(
                    query::Chain::This(query::Logic::This(query::Vec::Any(query::Logic::This(
                        query::Project3::Text(query::Logic::Or(
                            Box::new(query::Logic::Or(
                                Box::new(query::Logic::This(
                                    query::Text::Contains("fffffffffffffffffffffffffffffffff".into())
                                )),
                                Box::new(query::Logic::This(
                                    query::Text::Contains("x__0".into())
                                ))
                            )),
                            Box::new(query::Logic::Or(
                                Box::new(query::Logic::This(
                                    query::Text::Contains("Finai".into())
                                )),
                                Box::new(query::Logic::This(
                                    query::Text::Contains("Dyaros".into())
                                ))
                            )),
                        ))
                    ))))
                ))
            ],
            user_comments: vec![
                query::Result::Ok(query::Logic::This(
                    query::Chain::This(query::Logic::This(
                        query::Vec::Any(query::Logic::Or(
                            Box::new(query::Logic::Or(
                                Box::new(query::Logic::This(
                                    query::UserComment::AuthorName(query::Logic::This(
                                        query::Text::Is("fffffffffffffffffffffffffffffffff".to_string())
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
            ],
            user: vec![
                query::Result::Ok(query::Logic::This(
                    query::User::Profile(query::UserProfile::Bio(
                        query::Logic::Or(
                            Box::new(query::Logic::Or(
                                Box::new(query::Logic::This(
                                    query::Text::Contains("fffffffffffffffffffffffffffffffff".to_string())
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
            ],

        }
    };
    let api = s2rs::Api::new("");

    let mut locations = VecDeque::new();

    locations.push_back(location::User("x__0".into()).into());
    locations.push_back(location::UserProjects {
        name: "x__0".into(),
        next: Some(NextDirection::Up),
        page: 1
    }.into());
    locations.push_back(location::UserComments {
        name: "x__0".into(),
        next: Some(NextDirection::Up),
        page: 1
    }.into());
    
    let crawler = Crawler::new(locations, api);

    let mut bucket = Bucket {
        crawler,
        input
    };

    let mut file = File::options().append(true).open("RESULT").unwrap();

    while let Some(f) = bucket.next() {
        println!("Requesting - ");
        let data = f.await;
        match data {
            BucketOutput::Project(data) => if !data.queries.is_empty() {
                file.write_all(format!["https://scratch.mit.edu/projects/{}/\n", data.location.0].as_bytes()).unwrap();
            },
            BucketOutput::User(data) => if !data.queries.is_empty() {
                file.write_all(format!["https://scratch.mit.edu/users/{}/\n", data.location.0.as_str()].as_bytes()).unwrap();
            },
            BucketOutput::UserComments(data) => if !data.queries.is_empty() {
                file.write_all(format!["https://scratch.mit.edu/users/{}/ - comments {}\n", data.location.name.as_str(), data.location.page].as_bytes()).unwrap();
            },
            BucketOutput::UserProjects(data) => if !data.queries.is_empty() {
                file.write_all(format!["https://scratch.mit.edu/users/{}/ - projects {}\n", data.location.name.as_str(), data.location.page].as_bytes()).unwrap();
            },
            BucketOutput::Studio(data) => if !data.queries.is_empty() {
                file.write_all(format!["https://scratch.mit.edu/studios/{}/\n", data.location.0].as_bytes()).unwrap();
            },
            BucketOutput::StudioActivity(data) => if !data.queries.is_empty() {
                file.write_all(format!["https://scratch.mit.edu/studios/{}/ - activity {}\n", data.location.id, data.location.page].as_bytes()).unwrap();
            },
            BucketOutput::StudioComments(data) => if !data.queries.is_empty() {
                file.write_all(format!["https://scratch.mit.edu/studios/{}/ - comments {}\n", data.location.id, data.location.page].as_bytes()).unwrap();
            },
            BucketOutput::StudioProjects(data) => if !data.queries.is_empty() {
                file.write_all(format!["https://scratch.mit.edu/studios/{}/ - projects {}\n", data.location.id, data.location.page].as_bytes()).unwrap();
            },
            _ => { dbg![ "todo" ]; },
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }
}
