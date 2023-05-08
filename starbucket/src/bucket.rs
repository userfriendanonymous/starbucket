use std::{future::Future, sync::Arc};
use starcrawl::{crawler::Crawler, output::CrawlOutput};
use crate::{query::Query, input::Input, output::{BucketOutput, Output}};

pub struct Bucket {
    pub input: Input,
    pub crawler: Crawler
}

impl Bucket {
    pub fn next(&mut self) -> Option<impl Future<Output = BucketOutput> + '_> {
        let future = self.crawler.next()?;
        Some(async {
            let output = future.await;

            match output {
                CrawlOutput::User(output) => BucketOutput::User(Output::run(
                    output.capture, output.location, self.input.queries.user.clone()
                )),

                CrawlOutput::Project(output) => BucketOutput::Project(Output::run(
                    output.capture, output.location, self.input.queries.project.clone()
                )),

                CrawlOutput::UserComments(output) => BucketOutput::UserComments(Output::run(
                    output.capture, output.location, self.input.queries.user_comments.clone()
                )),

                CrawlOutput::UserProjects(output) => BucketOutput::UserProjects(Output::run(
                    output.capture, output.location, self.input.queries.user_projects.clone()
                )),

                CrawlOutput::UserFollowing(output) => BucketOutput::UserFollowing(Output::run(
                    output.capture, output.location, self.input.queries.user_following.clone()
                )),

                CrawlOutput::UserFollowers(output) => BucketOutput::UserFollowers(Output::run(
                    output.capture, output.location, self.input.queries.user_followers.clone()
                )),
                
                _ => BucketOutput::Todo
            }
        })
    }
}