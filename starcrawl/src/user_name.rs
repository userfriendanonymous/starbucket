
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct UserName(String);

impl UserName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for UserName {
    fn from(value: String) -> Self {
        Self(value.to_ascii_lowercase())
    }
}

impl From<&str> for UserName {
    fn from(value: &str) -> Self {
        Self(value.to_ascii_lowercase())
    }
}

impl From<UserName> for String {
    fn from(value: UserName) -> Self {
        value.0
    }
}
