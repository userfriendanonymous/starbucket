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
                    output.capture, output.location, &self.input.queries.user
                )),

                CrawlOutput::Project(output) => BucketOutput::Project(Output::run(
                    output.capture, output.location, &self.input.queries.project
                )),

                CrawlOutput::UserComments(output) => BucketOutput::UserComments(Output::run(
                    output.capture, output.location, &self.input.queries.user_comments
                )),

                CrawlOutput::UserProjects(output) => BucketOutput::UserProjects(Output::run(
                    output.capture, output.location, &self.input.queries.user_projects
                )),

                CrawlOutput::UserFollowing(output) => BucketOutput::UserFollowing(Output::run(
                    output.capture, output.location, &self.input.queries.user_following
                )),

                CrawlOutput::UserFollowers(output) => BucketOutput::UserFollowers(Output::run(
                    output.capture, output.location, &self.input.queries.user_followers
                )),

                CrawlOutput::Studio(output) => BucketOutput::Studio(Output::run(
                    output.capture, output.location, &self.input.queries.studio
                )),

                CrawlOutput::StudioActivity(output) => BucketOutput::StudioActivity(Output::run(
                    output.capture, output.location, &self.input.queries.studio_activity
                )),

                CrawlOutput::StudioComments(output) => BucketOutput::StudioComments(Output::run(
                    output.capture, output.location, &self.input.queries.studio_comments
                )),

                CrawlOutput::StudioProjects(output) => BucketOutput::StudioProjects(Output::run(
                    output.capture, output.location, &self.input.queries.studio_projects
                )),

                _ => BucketOutput::Todo
            }
        })
    }
}