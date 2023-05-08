use std::sync::Arc;
use crate::query;

pub struct Input {
    pub queries: InputQueries,
}

pub struct InputQueries {
    pub user: Arc<Vec<Arc<query::UserResult>>>,
    pub project: Arc<Vec<Arc<query::ProjectResult>>>,
    pub user_comments: Arc<Vec<Arc<query::UserCommentsResult>>>,
    pub user_projects: Arc<Vec<Arc<query::UserProjectsResult>>>,
    pub user_following: Arc<Vec<Arc<query::UserFollowingResult>>>,
    pub user_followers: Arc<Vec<Arc<query::UserFollowersResult>>>,
}