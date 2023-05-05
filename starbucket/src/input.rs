use std::sync::Arc;
use crate::query;

pub struct Input {
    pub queries: InputQueries,
}

pub struct InputQueries {
    pub user: Arc<Vec<Arc<query::UserResult>>>,
    pub project: Arc<Vec<Arc<query::ProjectResult>>>,
}