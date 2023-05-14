use std::sync::Arc;
use crate::query;

pub struct Input {
    pub queries: InputQueries,
}

pub struct InputQueries {
    pub user: Vec<query::UserResult>,
    pub project: Vec<query::ProjectResult>,
    pub studio: Vec<query::StudioResult>,
    pub user_comments: Vec<query::UserCommentsResult>,
    pub user_projects: Vec<query::UserProjectsResult>,
    pub user_following: Vec<query::UserFollowingResult>,
    pub user_followers: Vec<query::UserFollowersResult>,
    pub studio_activity: Vec<query::StudioActivityResult>,
    pub studio_comments: Vec<query::StudioCommentsResult>,
    pub studio_projects: Vec<query::StudioProjectsResult>,
}