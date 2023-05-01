
pub struct ActiveText {
    pub fragments: Vec<Fragment>
}

pub enum Fragment {
    Plain(String),
    UserLink(String),
    ProjectLink(u64),
    StudioLink(u64),
}
