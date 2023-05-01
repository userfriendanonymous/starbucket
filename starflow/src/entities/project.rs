

pub struct ProjectMeta {
    pub description: String,
    pub instructions: String,
    pub title: String,
    pub stats: ProjectStats,
    pub public: bool,
    pub history: ProjectHistory,
    pub comments_allowed: bool,
    pub author: ProjectAuthor,
    pub remix: ProjectRemix,
}

impl From<s2rs::api::Project> for ProjectMeta {
    fn from(value: s2rs::api::Project) -> Self {
        Self {
            description: value.description,
            instructions: value.instructions,
            stats: value.stats.into(),
            author: value.author.into(),
            comments_allowed: value.comments_allowed,
            history: value.history.into(),
            public: value.public,
            title: value.title,
            remix: value.remix.into()
        }
    }
}

pub struct ProjectStats {
    pub loves: u32,
    pub favorites: u32,
    pub remixes: u32,
    pub views: u32,
}

impl From<s2rs::api::ProjectStats> for ProjectStats {
    fn from(value: s2rs::api::ProjectStats) -> Self {
        Self {
            favorites: value.favorites as _,
            loves: value.loves as _,
            remixes: value.remixes as _,
            views: value.views as _
        }
    }
}

pub struct ProjectHistory {
    pub created: String,
    pub modified: String,
    pub shared: String,
}

impl From<s2rs::api::ProjectHistory> for ProjectHistory {
    fn from(value: s2rs::api::ProjectHistory) -> Self {
        Self {
            created: value.created,
            modified: value.modified,
            shared: value.shared
        }
    }
}

pub struct ProjectRemix {
    pub parent: Option<u64>,
    pub root: Option<u64>,
}

impl From<s2rs::api::ProjectRemix> for ProjectRemix {
    fn from(value: s2rs::api::ProjectRemix) -> Self {
        Self {
            parent: value.parent,
            root: value.root
        }
    }
}

pub struct ProjectAuthor {
    pub name: String,
    pub id: u64,
    pub scratch_team: bool,
}

impl From<s2rs::api::ProjectAuthor> for ProjectAuthor {
    fn from(value: s2rs::api::ProjectAuthor) -> Self {
        Self {
            id: value.id,
            name: value.name,
            scratch_team: value.scratch_team
        }
    }
}