/// Error types related to user management
#[derive(Debug)]
pub enum UserCreationError {
    /// The user already exists because its socket id is already taken
    UserAlreadyExists,
}

impl UserCreationError {
    pub fn to_string(&self) -> String {
        String::from(match self {
            UserCreationError::UserAlreadyExists => "UserAlreadyExists",
        })
    }
}
