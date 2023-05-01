use std::sync::Arc;
use crate::query;

pub struct Input {
    pub queries: InputQueries,
}

pub struct InputQueries {
    pub user_meta: Arc<Vec<Arc<query::UserMetaResult>>>,
    pub project_meta: Arc<Vec<Arc<query::ProjectMetaResult>>>,
}