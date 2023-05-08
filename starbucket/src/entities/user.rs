
// #[derive(Debug, Clone)]
// pub struct User {
//     pub name: String,
//     pub scratch_team: bool,
//     pub id: u64,
//     pub profile: UserProfile
// }

// impl From<s2rs::api::User> for User {
//     fn from(value: s2rs::api::User) -> Self {
//         Self {
//             profile: value.profile.into(),
//             id: value.id,
//             name: value.name,
//             scratch_team: value.scratch_team
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub struct UserProfile {
//     pub id: u64,
//     pub wiwo: String,
//     pub bio: String,
//     pub country: String,
// }

// impl From<s2rs::api::UserProfile> for UserProfile {
//     fn from(value: s2rs::api::UserProfile) -> Self {
//         Self {
//             bio: value.bio,
//             country: value.country,
//             id: value.id,
//             wiwo: value.status
//         }
//     }
// }

// pub struct UserProject {
//     pub description: String,
//     pub instructions: String,
// }


// pub struct UserComment {
//     pub id: u64,
//     pub profile_name: String,
//     pub author_name: String,
//     pub author_id: u64,
//     pub avatar_url: String,
//     pub content: CommentContent,
//     pub created_at: String,
//     pub replies: Vec<UserReply>,
// }

// impl From<s2rs::api::UserComment> for UserComment {
//     fn from(value: s2rs::api::UserComment) -> Self {
//         value.
//         Self {
//             content
//         }
//     }
// }

// pub struct UserReply {
//     pub id: u64,
//     pub profile_name: String,
//     pub author_name: String,
//     pub author_id: u64,
//     pub avatar_url: String,
//     pub content: CommentContent,
//     pub created_at: String,
// }