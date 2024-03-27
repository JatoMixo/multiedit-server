#[derive(Debug)]
pub enum UserCreationError {
    UserAlreadyExists,
}

impl UserCreationError {
    pub fn to_string(&self) -> String {
        String::from(match self {
            UserCreationError::UserAlreadyExists => "UserAlreadyExists",
        })
    }
}
