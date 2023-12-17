pub enum Role {
    System,
    User,
}

pub struct Message {
    pub content: String,
    pub role: Role,
}

impl Message {
    pub fn new(role: Role, content: &str) -> Self {
        Self {
            role,
            content: content.to_string(),
        }
    }
}
